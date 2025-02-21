use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum RunnerErrorType {
    UnsupportedLanguage,
    CodeRunFailed,
    FileNotFound,
}

impl fmt::Display for RunnerErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_description = match self {
            RunnerErrorType::UnsupportedLanguage => "Unsupported language",
            RunnerErrorType::CodeRunFailed => "Code run failed",
            RunnerErrorType::FileNotFound => "File not found",
        };

        write!(
            f,
            "[Runner Error] RunnerErrorType::{:?} {}",
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
