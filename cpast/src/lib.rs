//! # cpast - Code Testing and Analysis Tool
//!
//! `cpast` is a versatile code testing and analysis tool that empowers users in competitive programming and coding practice. It allows testing correct and incorrect code files against a custom language generator called `clex_gen`. This crate supports various programming languages, such as Python, C++, C, Rust, Ruby, JavaScript, and Java, and enables users to specify the number of iterations for testing code against random input values.
//!
//! ## Main Modules
//!
//! - `lang_runner`: Module for language-runner-related functionalities and handling the storage and management of code programs.
//! - `utils`: Utility module with miscellaneous functions.
//!
//! ## Introduction
//!
//! The crate provides solutions to common challenges faced by competitive programmers and coding enthusiasts, such as verifying code correctness, efficient testing under time constraints, and quick debugging to improve code performance.
//!
//! ## Usage
//!
//! To get started with `cpast`, users can use the provided functions:
//!
//! - `compile_and_test`: Compiles and tests code against a custom language generator.
//!
//! ## Example
//!
//! ```rust, no_run
//! use cpast::compile_and_test;
//!
//! async fn compile() {
//!     compile_and_test("correct.cpp".to_string(), "incorrect.rs".to_string(), "(N[1,10]) (?:N){\\1}".to_string(), 100, false, false, false).await.unwrap();
//! }
//! ```
//!
//! For more details on usage and advanced features, refer to the README.
//!

use ccode_runner::lang_runner::language_name::LanguageName;
use ccode_runner::lang_runner::runner::Language;
use colored::Colorize;
use error_types::cli_error::CliErrorType;
use futures::future::join_all;
use std::io;
use std::path::Path;
use std::sync::Arc;

use ccode_runner::lang_runner::program_store::ProgramStore;
use ccode_runner::lang_runner::runner_error_types::RunnerErrorType;
use clex_gen::clex_language::clex_error_type::ClexErrorType;
use clex_gen::clex_language::{code_generator, lexer, parser};
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) mod error_types;

pub const DEFAULT_FAIL_EXIT_CODE: i32 = 1;

#[derive(thiserror::Error, Debug)]
pub enum GenericCpastError {
    #[error("{0}")]
    ClexError(#[from] ClexErrorType),

    #[error("{0}")]
    RunnerError(#[from] Box<RunnerErrorType>),

    #[error("{0}")]
    CliError(#[from] CliErrorType),
}

pub enum CodeOrPath {
    /// Source Code, Clex
    Code(String, LanguageName),
    /// File Path
    Path(String),
}

/// Compile and test code against custom language generator.
///
/// # Arguments
///
/// * `correct_binding` - The source code file path containing correct code.
/// * `test_binding` - The source code file path containing incorrect code for testing.
/// * `language` - The custom language generator code for test generation.
/// * `iterations` - The number of test iterations to run.
/// * `no_stop` - Whether to stop after a failing testcase is found or not.
/// * `do_force_compile` - Whether to forcefully recompile files, even though it is updated
/// * `debug` - Whether to print debug information or not analogous to CPAST_DEBUG=1.
///
/// # Example
///
/// ```rust,no_run
/// async fn compile() {
///     cpast::compile_and_test("correct.cpp".to_string(), "incorrect.rs".to_string(), "(N[1,10]) (?:N){\\1}".to_string(), 100, false, false, false).await.unwrap();
/// }
/// ```
pub async fn compile_and_test(
    correct_binding: CodeOrPath,
    test_binding: String,
    language: String,
    iterations: usize,
    no_stop: bool,
    do_force_compile: bool,
    debug: bool,
) -> Result<(), GenericCpastError> {
    let store = match correct_binding {
        CodeOrPath::Code(correct_code, correct_lang) => {
            // CURRENTLY UNSTABLE
            eprintln!(
                "{}",
                "Using --problem_url is unstable at this moment. It may or may not work!"
                    .bold()
                    .red()
            );
            let correct_lang_instance =
                Language::new_from_text(&correct_code, correct_lang, do_force_compile)?;
            let test_lang_instance = Language::new(Path::new(&test_binding), do_force_compile)?;
            ProgramStore::new_from_language(correct_lang_instance, test_lang_instance)?
        }
        CodeOrPath::Path(correct_path) => ProgramStore::new(
            Path::new(&correct_path),
            Path::new(&test_binding),
            do_force_compile,
        )?,
    };
    let store = Arc::new(store);

    let mut token = lexer::Tokens::new(language);
    token.scan_tokens()?;
    let mut parser = parser::Parser::new_from_tokens(token);
    parser.parser()?;
    let generator = code_generator::Generator::new(&parser);
    let generator = Arc::new(generator);

    let has_failed = Arc::new(AtomicBool::new(false));
    // let semaphore = Arc::new(Semaphore::new(100)); // Limit concurrency to 64

    if debug {
        eprintln!("{}", "Debug mode enabled!".bold().yellow());
        eprintln!(
            "{}\n",
            "[INFO] Using multi-threading to speed up the process, testcase order might vary!"
                .bright_blue()
        );
    }

    let tasks = (1..=iterations)
        .map(|iter| {
            let has_failed_clone = Arc::clone(&has_failed);
            let store_clone = Arc::clone(&store);
            let generator_clone = Arc::clone(&generator);
            // let semaphore_clone = Arc::clone(&semaphore);

            tokio::spawn(async move {
                // let permit = semaphore_clone.acquire().await.unwrap(); // Acquire a permit
                process_test_case(
                    no_stop,
                    debug,
                    iter,
                    has_failed_clone,
                    store_clone,
                    generator_clone,
                )
                .await;
                // drop(permit);
            })
        })
        .collect::<Vec<_>>();

    join_all(tasks).await;

    if no_stop {
        println!(
            "\n{}",
            "Test case generation & matching done!".bold().bright_blue()
        );
    }

    if !has_failed.load(Ordering::Relaxed) {
        println!("{}", "🐣 Vohoo! No testcases have failed!".bold().green());
    }

    Ok(())
}

async fn process_test_case(
    no_stop: bool,
    debug: bool,
    iter: usize,
    has_failed_clone: Arc<AtomicBool>,
    store_clone: Arc<ProgramStore>,
    generator_clone: Arc<code_generator::Generator>,
) {
    if !no_stop && has_failed_clone.load(Ordering::Relaxed) {
        return;
    }

    match generator_clone.generate_testcases() {
        Err(err) => {
            eprintln!("{}", err);
            has_failed_clone.store(true, Ordering::Relaxed);
        }
        Ok(output_text) => match store_clone.run_codes_and_compare_output(&output_text) {
            Ok((true, _, _)) => {
                if !no_stop && debug {
                    eprintln!("{}", format!("Testcase {} ran successfully!", iter).green());
                }
            }
            Ok((false, expected, actual)) => {
                println!(
                    "{}\n{}\n{}\n==============================\n{}\n{}\n==============================\n{}\n{}",
                    format!("Testcase {} failed!", iter).red(),
                    "INPUT".underline(),
                    &output_text.cyan(),
                    "EXPECTED OUTPUT".underline(),
                    expected.green(),
                    "ACTUAL OUTPUT".underline(),
                    actual.red()
                );
                has_failed_clone.store(true, Ordering::Relaxed);
            }
            Err(err) => {
                eprintln!("{}", format!("Error matching the file! {}", err).red());
                if let RunnerErrorType::ProgramRunError(run_err) = *err {
                    if let Some(io_err) = run_err.downcast_ref::<io::Error>() {
                        if io_err.kind() == io::ErrorKind::BrokenPipe {
                            eprintln!("Broken pipe detected!");
                            eprintln!(
                                "This usually happens when your clex is incorrect and it doesn't generate what your codes are expecting!"
                            );
                            eprintln!("Please check your clex and try again!");
                        }
                    }
                }

                has_failed_clone.store(true, Ordering::Relaxed);
            }
        },
    }
}
