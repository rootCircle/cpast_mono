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

/// Error type that includes source context for better error messages
#[derive(Debug)]
pub struct ClexError {
    error: ClexErrorType,
    source: String,
}

impl ClexError {
    /// Create a new ClexError with source context
    pub fn new(error: ClexErrorType, source: String) -> Self {
        Self { error, source }
    }

    /// Get the underlying error
    pub fn error(&self) -> &ClexErrorType {
        &self.error
    }

    /// Get the source string
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Format the error with source context (cargo/clippy style)
    pub fn format_with_context(&self) -> String {
        self.error.format_with_source(&self.source)
    }
}

impl From<ClexErrorType> for ClexError {
    fn from(error: ClexErrorType) -> Self {
        Self {
            error,
            source: String::new(),
        }
    }
}

impl std::fmt::Display for ClexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.format_with_context())
    }
}

impl std::error::Error for ClexError {}

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
pub fn get_tokens(language: String) -> Result<Vec<Token>, ClexError> {
    let source = language.clone();
    let mut token = lexer::Tokens::new(language);
    token.scan_tokens().map_err(|e| ClexError::new(e, source))?;
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
pub fn get_ast(language: String) -> Result<ClexLanguageAST, ClexError> {
    let source = language.clone();
    let mut parser =
        parser::Parser::new(language).map_err(|e| ClexError::new(e, source.clone()))?;
    parser.parser().map_err(|e| ClexError::new(e, source))?;
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
pub fn generator(language: String) -> Result<String, ClexError> {
    let source = language.clone();
    let mut parser =
        parser::Parser::new(language).map_err(|e| ClexError::new(e, source.clone()))?;
    parser
        .parser()
        .map_err(|e| ClexError::new(e, source.clone()))?;
    let generator = code_generator::Generator::new(&parser);
    generator
        .generate_testcases()
        .map_err(|e| ClexError::new(e, source))
}
