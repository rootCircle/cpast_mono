//! This module defines the `RunnerErrorType` enum, which represents various
//! error types that can occur during code execution in a runner environment.
//!
//! The enum includes variants for unsupported languages, invalid language
//! mappings, compilation failures, file not found errors, and runtime errors.
//! Each variant provides specific information to aid in error handling and
//! debugging. The module also implements the `std::fmt::Display` and
//! `std::error::Error` traits for formatted error messages and compatibility
//! with Rust's error handling system.
//!

use core::fmt;
use std::{error::Error, path::PathBuf};

use super::language_name::{CompilationType, LanguageName};

/// Represents errors that can occur during the execution of a code runner.
///
/// This enum encapsulates various error types that may arise when processing
/// code files, including issues related to unsupported languages, compilation
/// failures, and runtime errors. Each variant provides specific information
/// about the nature of the error encountered.
#[derive(Debug)]
pub enum RunnerErrorType {
    /// Indicates that the provided file has an unsupported language extension.
    ///
    /// The associated `PathBuf` contains the path to the unsupported file.
    UnsupportedLanguage(PathBuf),

    /// Indicates an invalid mapping between a programming language and its
    /// compilation type.
    ///
    /// The associated `LanguageName` and `CompilationType` provide details
    /// about the language and the attempted compilation type that failed.
    InvalidLanguageMapping(LanguageName, CompilationType),

    /// Indicates that no valid compilation configuration was found for the
    /// specified programming language.
    ///
    /// The associated `LanguageName` provides the language that could not
    /// be mapped to a compilation configuration.
    InvalidCompilationMapping(LanguageName),

    /// Indicates that the code execution failed after all available compilers
    /// and runners encountered errors.
    ///
    /// The associated `PathBuf` contains the path to the code file that failed
    /// to execute. The user is advised to verify the code and its compatibility
    /// with the target environment.
    CodeRunFailed(PathBuf),

    /// Indicates that the specified file could not be found.
    ///
    /// The associated `PathBuf` contains the path to the missing file.
    FileNotFound(PathBuf),

    /// Indicates that a warmup compilation was not completed before attempting
    /// to run the code.
    ///
    /// This error suggests that the user should call `warmup_precompile()`
    /// before running the code.
    WarmupCompileFatal,

    /// Indicates a runtime error occurred during the execution of the program.
    ///
    /// The associated `Box<dyn Error + Send + Sync>` contains the underlying
    /// error that caused the runtime failure.
    ProgramRunError(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for RunnerErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_description = match self {
            RunnerErrorType::UnsupportedLanguage(filepath) => format!(
                "Cannot process file with unsupported language extension: {}",
                filepath.display()
            ),
            RunnerErrorType::InvalidLanguageMapping(lang, compilation_type) => format!(
                "Cannot map language '{:?}' to compilation type '{:?}'",
                lang, compilation_type
            ),
            RunnerErrorType::InvalidCompilationMapping(lang) => format!(
                "No valid compilation configuration found for language '{:?}'",
                lang
            ),
            RunnerErrorType::CodeRunFailed(filepath) => {
                format!(
                    "Failed to execute code at '{}'. All available compilers and runners encountered errors. Please verify the code and ensure it's compatible with the target environment.",
                    filepath.display()
                )
            }
            RunnerErrorType::FileNotFound(filepath) => {
                format!("{} file could not be found", filepath.display())
            }
            RunnerErrorType::WarmupCompileFatal => String::from(
                "Fatal Error: Compilation environment not initialized. Call warmup_precompile() first",
            ),
            RunnerErrorType::ProgramRunError(err) => format!("Runtime error occurred: {}", err),
        };

        write!(
            f,
            "[RUNNER ERROR] RunnerErrorType::{:?} {}",
            self, error_description
        )
    }
}

impl Error for RunnerErrorType {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
