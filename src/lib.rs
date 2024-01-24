//! # cpast - Code Testing and Analysis Tool
//!
//! `cpast` is a versatile code testing and analysis tool that empowers users in competitive programming and coding practice. It allows testing correct and incorrect code files against a custom language generator called `clex`. This crate supports various programming languages, such as Python, C++, C, Rust, Ruby, JavaScript, and Java, and enables users to specify the number of iterations for testing code against random input values.
//!
//! ## Main Modules
//!
//! - `language`: Module for language-runner-related functionalities.
//! - `program_store`: Module handling the storage and management of code programs.
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
//! compile_and_test("correct.cpp".to_string(), "incorrect.cpp".to_string(), "(N) (?:N){\\1}".to_string(), 100, false, false);
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
mod utils;
mod lang_runner;

use futures::future::join_all;
use std::path::Path;
use std::process::exit;

use crate::clex_language::lexer::Token;
use crate::clex_language::parser::Parser;
use crate::clex_language::{ast::Program, generator, lexer, parser};
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
/// * `do_force_compile` - Whether or not to forcefully recompile files, even though it is updated
///
/// # Example
///
/// ```rust,no_run
/// cpast::compile_and_test("correct.cpp".to_string(), "incorrect.rs".to_string(), "(N[1,10]) (?:N){\\1}".to_string(), 100, false, false);
/// ```
pub async fn compile_and_test(
    correct_binding: String,
    test_binding: String,
    language: String,
    iterations: usize,
    no_stop: bool,
    do_force_compile: bool,
) {

    let store = ProgramStore::new(
        Path::new(&correct_binding),
        Path::new(&test_binding),
        do_force_compile,
    );

    let store: &'static ProgramStore = Box::leak(store.into());


    let mut token = lexer::Tokens::new(language);
    token.scan_tokens();

    let mut parser = parser::Parser::new_from_tokens(token);
    parser.parser();

    let parser: &'static Parser = Box::leak(parser.into());

    let tasks = (1..=iterations)
        .map(|iter| {
            tokio::spawn(async move {
                let mut gen = generator::Generator::new(parser.to_owned());

                gen.traverse_ast();

                match store.run_code(&gen.output_text) {
                    Ok((true, _, _)) => println!("Testcase {iter} ran successfully!"),
                    Ok((false, expected, actual)) => {
                        println!("Testcase {iter} failed!");
                        println!("INPUT\n{}", &gen.output_text);
                        println!("==============================");
                        println!("EXPECTED OUTPUT\n{expected}");
                        println!("==============================");
                        println!("ACTUAL OUTPUT\n{actual}");
                        if !no_stop {
                            exit(0);
                        }
                    }
                    Err(err) => println!("Error matching the file! {err}"),
                }

                gen.reset_output();
            })
        })
        .collect::<Vec<_>>();

    join_all(tasks).await;
}

/// Get tokens from the custom language lexer.
///
/// # Arguments
///
/// * `language` - The custom language generator code for test generation.
///
/// # Returns
///
/// A vector of `Token` representing the lexed tokens.
///
/// # Example
///
/// ```rust
/// let tokens = cpast::get_tokens("(N) (?:N){\\1}".to_string());
/// ```
#[must_use]
pub fn get_tokens(language: String) -> Vec<Token> {
    let mut token = lexer::Tokens::new(language);
    token.scan_tokens();
    token.tokens
}

/// Get the Abstract Syntax Tree (AST) from the custom language parser.
///
/// # Arguments
///
/// * `language` - The custom language generator code for test generation.
///
/// # Returns
///
/// The `Program` AST representing the parsed program.
///
/// # Example
///
/// ```rust
/// let ast = cpast::get_ast("(N) (?:N){\\1}".to_string());
/// ```
#[must_use]
pub fn get_ast(language: String) -> Program {
    let mut parser = parser::Parser::new(language);
    parser.parser();
    parser.language
}

/// Generate code based on the custom language specification.
///
/// # Arguments
///
/// * `language` - The custom language generator code for test generation.
///
/// # Returns
///
/// A string representing the generated test pattern.
///
/// # Example
///
/// ```rust
/// let generated_code = cpast::generator("(N[1,10]) (?:N){\\1}".to_string());
/// ```
#[must_use]
pub fn generator(language: String) -> String {
    let mut parser = parser::Parser::new(language);
    parser.parser();
    let mut gen = generator::Generator::new(parser);
    gen.traverse_ast();
    gen.output_text
}
