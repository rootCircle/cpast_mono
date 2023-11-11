//! The `clex_language` module encompasses components related to the custom language generator, known as `clex`.
//! It includes modules for Abstract Syntax Tree (AST) definition, lexer, parser, and the generator itself.
//! The `clex` language is designed for defining input patterns and specifications for code testing and analysis.
//!
//! # Modules
//!
//! - `ast`: Defines the Abstract Syntax Tree (AST) for the `clex` language, representing the structure of code patterns.
//! - `generator`: Implements the code generator responsible for producing code based on the `clex` language specification.
//! - `lexer`: Provides lexical analysis capabilities for tokenizing input patterns in the `clex` language.
//! - `parser`: Implements the parser for interpreting and structuring the `clex` language into an Abstract Syntax Tree (AST).
//!
//! The `clex` language allows users to specify various data types, repetition patterns, and capturing groups,
//! providing a flexible way to define input patterns for code testing and analysis.
//! For detailed usage and syntax rules, refer to the Grammar Rules for Clex Generator in README.
pub mod ast;
pub(crate) mod generator;
pub mod lexer;
pub(crate) mod parser;
