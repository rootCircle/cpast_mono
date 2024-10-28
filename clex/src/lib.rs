//! Clex - Custom Language Generator
//!
//! This module provides functionality to work with a custom language generator designed for creating test patterns.
//! It includes methods for tokenizing, parsing, and generating code based on the custom language specification.
//!
//! ## Main Modules
//!
//! - `clex_language`: Module containing lexer, parser, generator, and abstract syntax tree (AST) for the custom language `clex`.
//! ## Usage
//!
//! To use the `clex` module, add it as a dependency in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! clex = "0.1"  // Adjust the version as necessary
//! ```
//!
//! Import the module in your Rust code:
//!
//! ```rust
//! use clex::{get_tokens, get_ast, generator};
//! ```
//!
//! ## Example
//!
//! Here’s a complete example demonstrating how to use the functions provided by the `clex` module:
//!
//! ```rust
//! use clex::{get_tokens, get_ast, generator};
//! // Get tokens from custom language
//! let tokens = get_tokens("(N) (?:N){\\1}".to_string()).unwrap();
//! println!("Tokens: {:?}", tokens);
//!
//! // Get the Abstract Syntax Tree (AST)
//! let ast = get_ast("(N) (?:N){\\1}".to_string()).unwrap();
//! println!("AST: {:?}", ast);
//!
//! // Generate code based on the custom language specification
//! let generated_code = generator("(N[1,10]) (?:N){\\1}".to_string()).unwrap();
//! println!("Generated Code: {}", generated_code);
//! ```
//!
//! ## Modules
//!
//! This module consists of the following sub-modules:
//!
//! - `lexer`: Responsible for tokenizing the input language.
//! - `parser`: Handles the parsing of tokens to generate the Abstract Syntax Tree (AST).
//! - `generator`: Generates test patterns based on the parsed language specifications.
pub mod clex_language;
use crate::clex_language::clex_error_type::ClexErrorType;
use crate::clex_language::lexer::Token;
use crate::clex_language::{ast::ClexLanguageAST, code_generator, lexer, parser};

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
/// let tokens = clex::get_tokens("(N) (?:N){\\1}".to_string()).unwrap();
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
/// let ast = clex::get_ast("(N) (?:N){\\1}".to_string()).unwrap();
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
/// let generated_code = clex::generator("(N[1,10]) (?:N){\\1}".to_string()).unwrap();
/// ```
pub fn generator(language: String) -> Result<String, ClexErrorType> {
    let mut parser = parser::Parser::new(language)?;
    parser.parser()?;
    let mut gen = code_generator::Generator::new(parser);
    gen.generate_testcases()
}
