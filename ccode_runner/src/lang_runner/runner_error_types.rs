use core::fmt;
use std::{error::Error, path::PathBuf};

use super::language_name::{CompilationType, LanguageName};

#[derive(Debug)]
pub enum RunnerErrorType {
    UnsupportedLanguage(PathBuf),
    InvalidLanguageMapping(LanguageName, CompilationType),
    InvalidCompilationMapping(LanguageName),
    CodeRunFailed(PathBuf),
    FileNotFound(PathBuf),
    /// Warmup compilation is not done before running the code
    WarmupCompileFatal,

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
