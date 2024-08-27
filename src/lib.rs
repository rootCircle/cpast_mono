//! # cpast - Code Testing and Analysis Tool
//!
//! `cpast` is a versatile code testing and analysis tool that empowers users in competitive programming and coding practice. It allows testing correct and incorrect code files against a custom language generator called `clex`. This crate supports various programming languages, such as Python, C++, C, Rust, Ruby, JavaScript, and Java, and enables users to specify the number of iterations for testing code against random input values.
//!
//! ## Main Modules
//!
//! - `lang_runner`: Module for language-runner-related functionalities and handling the storage and management of code programs.
//! - `utils`: Utility module with miscellaneous functions.
//! - `clex_language`: Module containing lexer, parser, generator, and abstract syntax tree (AST) for the custom language `clex`.
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
//! - `get_tokens`: Retrieves tokens from the custom language lexer.
//! - `get_ast`: Obtains the Abstract Syntax Tree (AST) from the custom language parser.
//! - `generator`: Generates code based on the custom language specification.
//!
//! ## Example
//!
//! ```rust, no_run
//! use cpast::{compile_and_test, get_tokens, get_ast, generator};
//!
//! async fn compile() {
//!     compile_and_test("correct.cpp".to_string(), "incorrect.rs".to_string(), "(N[1,10]) (?:N){\\1}".to_string(), 100, false, false).await.unwrap();
//! }
//!
//! let tokens = get_tokens("(N) (?:N){\\1}".to_string());
//!
//! let ast = get_ast("(N) (?:N){\\1}".to_string());
//!
//! let generated_code = generator("(N) (?:N){\\1}".to_string());
//! ```
//!
//! For more details on usage and advanced features, refer to the README.
//!

pub mod clex_language;
mod lang_runner;
mod utils;

use colored::Colorize;
use futures::future::join_all;
use std::path::Path;
use std::process::exit;
use std::sync::{Arc, Mutex};

use crate::clex_language::clex_error_type::ClexErrorType;
use crate::clex_language::lexer::Token;
use crate::clex_language::parser::Parser;
use crate::clex_language::{ast::ClexLanguageAST, generator, lexer, parser};
use crate::lang_runner::program_store::ProgramStore;

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
///
/// # Example
///
/// ```rust,no_run
/// async fn compile() {
///     cpast::compile_and_test("correct.cpp".to_string(), "incorrect.rs".to_string(), "(N[1,10]) (?:N){\\1}".to_string(), 100, false, false).await.unwrap();
/// }
/// ```
pub async fn compile_and_test(
    correct_binding: String,
    test_binding: String,
    language: String,
    iterations: usize,
    no_stop: bool,
    do_force_compile: bool,
) -> Result<(), ClexErrorType> {
    let store = ProgramStore::new(
        Path::new(&correct_binding),
        Path::new(&test_binding),
        do_force_compile,
    );

    let store: &'static ProgramStore = Box::leak(store.into());

    let mut token = lexer::Tokens::new(language);
    token.scan_tokens()?;

    let mut parser = parser::Parser::new_from_tokens(token);
    parser.parser()?;

    let parser: &'static Parser = Box::leak(parser.into());

    // Storing state if testcase matching has failed or not
    let has_failed = Arc::new(Mutex::new(false));

    println!(
        "{}\n",
        "[INFO] Using multi-threading to speed up the process, testcase order might vary!"
            .bright_blue()
    );

    let tasks = (1..=iterations)
        .map(|iter| {
            let has_failed_clone = Arc::clone(&has_failed);
            tokio::spawn(async move {
                let mut gen = generator::Generator::new(parser.to_owned());

                let output_text = gen.generate_testcases().unwrap_or_else(|err| {
                    err.print_and_exit()
                });

                match store.run_codes_and_compare_output(&output_text) {
                    Ok((true, _, _)) => {
                        if !no_stop {
                            println!("{}", format!("Testcase {} ran successfully!", iter).green());
                        }
                    }
                    Ok((false, expected, actual)) => {
                        // Each usage of println!() puts a lock on stdout
                        println!("{}\n{}\n{}\n==============================\n{}\n{}\n==============================\n{}\n{}",
                                 format!("Testcase {} failed!", iter).red(),
                                 "INPUT".underline(),
                                 &output_text.cyan(),
                                 "EXPECTED OUTPUT".underline(),
                                 expected.green(),
                                 "ACTUAL OUTPUT".underline(),
                                 actual.red());


                        if !no_stop {
                            exit(0);
                        }
                        else {
                            let mut has_failed_guard = has_failed_clone.lock().unwrap();
                            *has_failed_guard = true;
                            drop(has_failed_guard);

                        }
                    }
                    Err(err) => {
                        println!("{}", format!("Error matching the file! {}", err).red())
                    }
                }
            })
        })
        .collect::<Vec<_>>();

    join_all(tasks).await;

    if no_stop {
        println!(
            "\n{}",
            "Test case generation & matching done!".bold().bright_blue()
        );

        let has_failed_clone = Arc::clone(&has_failed);
        let has_failed_guard = has_failed_clone.lock().unwrap();

        if !*has_failed_guard {
            println!(
                "{}",
                "All testcases ran successfully! No failures detected."
                    .bold()
                    .green()
            );
        }
    } else {
        println!(
            "\n{}",
            "All testcases ran successfully! No failures detected."
                .bold()
                .green()
        );
    }

    Ok(())
}

/// Get tokens from the custom language lexer.
///
/// # Arguments
///
/// * `language` - The custom language generator code for test generation.
///
/// # Returns
///
/// Result enum, if Ok contains a vector of `Token` representing the lexed tokens.
///
/// # Example
///
/// ```rust
/// let tokens = cpast::get_tokens("(N) (?:N){\\1}".to_string()).unwrap();
/// ```
pub fn get_tokens(language: String) -> Result<Vec<Token>, ClexErrorType> {
    let mut token = lexer::Tokens::new(language);
    token.scan_tokens()?;
    Ok(token.get_tokens())
}

/// Get the Abstract Syntax Tree (AST) from the custom language parser.
///
/// # Arguments
///
/// * `language` - The custom language generator code for test generation.
///
/// # Returns
///
/// Result enum, if Ok contains the `ClexLanguageAST` AST representing the parsed program.
///
/// # Example
///
/// ```rust
/// let ast = cpast::get_ast("(N) (?:N){\\1}".to_string()).unwrap();
/// ```
pub fn get_ast(language: String) -> Result<ClexLanguageAST, ClexErrorType> {
    let mut parser = parser::Parser::new(language)?;
    parser.parser()?;
    Ok(parser.get_language().clone())
}

/// Generate code based on the custom language specification.
///
/// # Arguments
///
/// * `language` - The custom language generator code for test generation.
///
/// # Returns
///
/// Result enum, if Ok contains a string representing the generated test pattern.
///
/// # Example
///
/// ```rust
/// let generated_code = cpast::generator("(N[1,10]) (?:N){\\1}".to_string()).unwrap();
/// ```
pub fn generator(language: String) -> Result<String, ClexErrorType> {
    let mut parser = parser::Parser::new(language)?;
    parser.parser()?;
    let mut gen = generator::Generator::new(parser);
    gen.generate_testcases()
}
