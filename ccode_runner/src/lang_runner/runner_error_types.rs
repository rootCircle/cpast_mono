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
    /// Indicates that the provided file extension can't be determined.
    /// Mostly because the file extension has no extension. This is very common in UNIX environments.
    ///
    /// The associated `PathBuf` contains the path to the file with no extension.
    InvalidFileExtension(PathBuf),

    /// Indicates that the provided file has an invalid name, may contain some non-parseable characters.
    ///
    /// The associated `PathBuf` contains the path to the invalid file.
    InvalidFileName(PathBuf),

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
    /// This error suggests that the user should call `compile_language()`
    /// before running the code.
    WarmupCompileFatal,

    /// Indicates a runtime error occurred during the execution of the program.
    ///
    /// The associated `Box<dyn Error + Send + Sync>` contains the underlying
    /// error that caused the runtime failure.
    ProgramRunError(Box<dyn Error + Send + Sync>),

    /// Indicates an error occurred while creating a temporary file.
    ///
    /// The associated `Box<dyn Error + Send + Sync>` contains the underlying
    /// error that occurred during file creation.
    FileCreationError(Box<dyn Error + Send + Sync>),

    /// Indicates that the destination path for the compiled code is None. It is expected to be Some(PathBuf).
    ///
    /// The associated `PathBuf`, `LanguageName`, and `CompilationType` provide details about the missing destination path.
    EmptyDestinationPath(PathBuf, LanguageName, CompilationType),

    /// Indicates that the temporary directory is empty. It is expected to contain the compiled code.
    ///
    /// The associated `PathBuf`, `LanguageName`, and `CompilationType` provide details about the empty temporary directory.
    EmptyTempDir(PathBuf, LanguageName, CompilationType),

    /// Indicates that the file stem could not be extracted.
    ///
    /// The associated `PathBuf` contains the path to the source file.
    FileStemExtractionError(PathBuf),

    /// Indicates that a Java file does not contain a public class.
    ///
    /// The associated `String` contains the content of the Java file.
    JavaNoPublicClassFound(String),

    /// Indicates that the destination and source file stem differ in Java.
    /// Java has strict rules for file naming and the destination file must match the source file.
    ///
    /// The associated `Option<PathBuf>` contains the path to the destination file
    JavaMismatchDestinationFile(Option<PathBuf>),

    /// Indicates that the program execution exceeded the specified time limit.
    ///
    /// The associated `u64` contains the time limit in milliseconds.
    TimeLimitExceeded(u64),

    /// Indicates that the program execution exceeded the specified memory limit.
    ///
    /// The associated `u64` contains the memory limit in bytes.
    MemoryLimitExceeded(u64),
}

impl fmt::Display for RunnerErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_description = match self {
            RunnerErrorType::InvalidFileExtension(path_buf) => {
                format!(
                    "Cannot determine file extension for file: {}",
                    path_buf.display()
                )
            }
            RunnerErrorType::InvalidFileName(path_buf) => {
                format!("Invalid file name for file: {}", path_buf.display())
            }
            RunnerErrorType::UnsupportedLanguage(filepath) => format!(
                "Cannot process file with unsupported language extension: {}",
                filepath.display()
            ),
            RunnerErrorType::InvalidLanguageMapping(lang, compilation_type) => {
                format!("Cannot map language '{lang:?}' to compilation type '{compilation_type:?}'")
            }
            RunnerErrorType::InvalidCompilationMapping(lang) => {
                format!("No valid compilation configuration found for language '{lang:?}'")
            }
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
                "Fatal Error: Compilation environment not initialized. Call compile_language() first",
            ),
            RunnerErrorType::ProgramRunError(err) => format!("Runtime error occurred: {err}"),
            RunnerErrorType::FileCreationError(err) => format!("Error creating tempfile: {err}"),
            RunnerErrorType::EmptyDestinationPath(path_buf, lang, compilation_type) => format!(
                "Destination path is None for file: {}. Expected Some(PathBuf) for language '{:?}' and compilation type '{:?}'",
                path_buf.display(),
                lang,
                compilation_type
            ),
            RunnerErrorType::EmptyTempDir(path_buf, lang, compilation_type) => format!(
                "Temporary directory is empty for file: {}. Expected to contain the compiled code for language '{:?}' and compilation type '{:?}'",
                path_buf.display(),
                lang,
                compilation_type
            ),
            RunnerErrorType::FileStemExtractionError(path_buf) => format!(
                "Error extracting file stem for file: {}",
                path_buf.display()
            ),
            RunnerErrorType::JavaNoPublicClassFound(content) => {
                format!("No public class found in Java file: {content}")
            }
            RunnerErrorType::JavaMismatchDestinationFile(path_buf) => {
                format!(
                    "Destination file stem does not match source file stem for Java file: {path_buf:?}"
                )
            }
            RunnerErrorType::TimeLimitExceeded(time_limit_ms) => {
                format!("Program execution exceeded the time limit of {time_limit_ms} milliseconds")
            }
            RunnerErrorType::MemoryLimitExceeded(memory_limit_bytes) => {
                format!("Program execution exceeded the memory limit of {memory_limit_bytes} bytes")
            }
        };

        write!(
            f,
            "[RUNNER ERROR] RunnerErrorType::{self:?} {error_description}"
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
