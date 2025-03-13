//! Provides functionality for handling programming language names and their associated file extensions.
//!
//! This module manages the classification, validation, and conversion of programming language identifiers
//! and their corresponding compilation types. It offers a type-safe way to work with supported programming
//! languages in the system.
//!
//! # Features
//!
//! * Language name enumeration with serialization support
//! * File extension to language name mapping
//! * Compilation type classification
//!
//! # Types
//!
//! - [`LanguageName`]: Enumeration of supported programming languages
//! - [`CompilationType`]: Classification of how languages are executed
//!
//! # Supported Languages
//!
//! - Python (.py)
//! - C++ (.cpp, .cxx, .c++, .cc, .C)
//! - C (.c)
//! - Rust (.rs)
//! - Ruby (.rb)
//! - JavaScript (.js)
//! - Java (.java)
//!
//! This module includes utilities for:
//! - Converting between file extensions and language names
//! - Validating language support based on file paths
//! - Formatting language names for display
//!

use std::{fmt, path::Path};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::runner_error_types::RunnerErrorType;

/// Enumeration of supported programming languages.
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, PartialEq)]
pub enum LanguageName {
    Python,
    Cpp,
    C,
    Rust,
    Ruby,
    Javascript,
    Java,
}

impl fmt::Display for LanguageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LanguageName::Python => write!(f, "python"),
            LanguageName::Cpp => write!(f, "cpp"),
            LanguageName::C => write!(f, "c"),
            LanguageName::Rust => write!(f, "rust"),
            LanguageName::Ruby => write!(f, "ruby"),
            LanguageName::Javascript => write!(f, "javascript"),
            LanguageName::Java => write!(f, "java"),
        }
    }
}

impl TryFrom<String> for LanguageName {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "python" => Ok(LanguageName::Python),
            "cpp" => Ok(LanguageName::Cpp),
            "c" => Ok(LanguageName::C),
            "rust" => Ok(LanguageName::Rust),
            "ruby" => Ok(LanguageName::Ruby),
            "javascript" => Ok(LanguageName::Javascript),
            "java" => Ok(LanguageName::Java),
            other => Err(format!(
                "{} is not a supported language. Use either `python`, `cpp`, `c`, `rust`, `ruby`, `javascript` or `java`.",
                other
            )),
        }
    }
}

/// Classification of how programming languages are executed.
/// This enum categorizes languages based on their compilation and execution model.
/// Helps in optimizing on repeated compilations and executions.
#[derive(Debug, PartialEq, Clone)]
pub enum CompilationType {
    Compiled,         // Compiled language like C, C++, Rust, Go, etc.
    Interpreted,      // Interpreted language like Python, etc.
    BytecodeCompiled, // Java, compiled to bytecode, executed by JVM
}

impl TryFrom<&Path> for LanguageName {
    type Error = RunnerErrorType;

    fn try_from(file_path: &Path) -> Result<Self, Self::Error> {
        match file_path.extension().and_then(|ext| ext.to_str()) {
            Some("rs") => Ok(LanguageName::Rust),
            Some("py") => Ok(LanguageName::Python),
            Some("c") => Ok(LanguageName::C),
            Some("cpp") | Some("cxx") | Some("c++") | Some("cc") | Some("C") => {
                Ok(LanguageName::Cpp)
            }
            Some("java") => Ok(LanguageName::Java),
            Some("js") => Ok(LanguageName::Javascript),
            Some("rb") => Ok(LanguageName::Ruby),
            Some(_) => Err(RunnerErrorType::UnsupportedLanguage(
                file_path.to_path_buf(),
            )),
            None => Err(RunnerErrorType::InvalidFileExtension(
                file_path.to_path_buf(),
            )),
        }
    }
}

pub(super) fn get_programming_language_name(file_path: &Path) -> Option<LanguageName> {
    LanguageName::try_from(file_path).ok()
}

impl LanguageName {
    /// Gets the default file extension for this programming language.
    pub fn file_extension(&self) -> &'static str {
        match self {
            LanguageName::Rust => "rs",
            LanguageName::Python => "py",
            LanguageName::C => "c",
            LanguageName::Cpp => "cpp",
            LanguageName::Java => "java",
            LanguageName::Javascript => "js",
            LanguageName::Ruby => "rb",
        }
    }
}

pub(super) fn get_file_extension(lang_name: &LanguageName) -> &'static str {
    lang_name.file_extension()
}

pub(super) fn get_language_compilation_type(lang_name: &LanguageName) -> CompilationType {
    match lang_name {
        LanguageName::Rust | LanguageName::Cpp | LanguageName::C => CompilationType::Compiled,
        LanguageName::Python | LanguageName::Ruby | LanguageName::Javascript => {
            CompilationType::Interpreted
        }
        LanguageName::Java => CompilationType::BytecodeCompiled,
    }
}
