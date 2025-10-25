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
//! To use the `clex_gen` module, add it as a dependency in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! clex_gen = "0.1"
//! ```
//!
//! Import the module in your Rust code:
//!
//! ```rust
//! use clex_gen::{get_tokens, get_ast, generator};
//! ```
//!
//! ## Example
//!
//! Here’s a complete example demonstrating how to use the functions provided by the `clex` module:
//!
//! ```rust
//! use clex_gen::{get_tokens, get_ast, generator};
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
/// let tokens = clex_gen::get_tokens("(N) (?:N){\\1}".to_string()).unwrap();
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
/// let ast = clex_gen::get_ast("(N) (?:N){\\1}".to_string()).unwrap();
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
/// let generated_code = clex_gen::generator("(N[1,10]) (?:N){\\1}".to_string()).unwrap();
/// ```
pub fn generator(language: String) -> Result<String, ClexErrorType> {
    let mut parser = parser::Parser::new(language)?;
    parser.parser()?;
    let generator = code_generator::Generator::new(&parser);
    generator.generate_testcases()
}

/// Generate code as an iterator based on the custom language specification.
///
/// This function returns an iterator that yields chunks of the generated test pattern
/// incrementally, processing one unit expression at a time. This is more memory-efficient
/// for very large test cases (in the order of GiBs) compared to generating the entire
/// test case at once.
///
/// # Arguments
///
/// * `language` - The custom language generator code for test generation.
///
/// # Returns
///
/// Result enum containing an iterator that yields `Result<String, ClexErrorType>` for each chunk.
///
/// # Example
///
/// ```rust
/// use clex_gen::generator_iter;
///
/// let mut output = String::new();
/// for chunk in generator_iter("N[1,10] N[1,10]".to_string()).unwrap() {
///     match chunk {
///         Ok(data) => output.push_str(&data),
///         Err(e) => {
///             eprintln!("Error: {}", e);
///             break;
///         }
///     }
/// }
/// // Remove trailing space if needed
/// if output.ends_with(' ') {
///     output.pop();
/// }
/// println!("{}", output);
/// ```
pub fn generator_iter(
    language: String,
) -> Result<code_generator::TestCaseIterator, ClexErrorType> {
    let mut parser = parser::Parser::new(language)?;
    parser.parser()?;
    let generator = code_generator::Generator::new(&parser);
    Ok(generator.generate_testcases_iter())
}
