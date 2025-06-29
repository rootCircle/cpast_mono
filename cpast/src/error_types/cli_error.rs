use core::fmt;
use std::error::Error;

use cscrapper::qscrapper::ScraperError;

#[derive(Debug)]
pub enum CliErrorType {
    GeminiAPIKeyMissing,
    ClexLLMInitilizationError(Box<dyn Error>),
    ClexLLMGenerationError(Box<dyn Error>),
    InputFormatMissing,
    ConstraintsMissing,
    AiRequiredArgsMissing,
    CScrapperError(ScraperError),
}

impl fmt::Display for CliErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_description = match self {
            CliErrorType::GeminiAPIKeyMissing => {
                "Gemini API key is missing. \
                Please set the environment variable 'GOOGLE_API_KEY' with the API key.\n\n\
                export GOOGLE_API_KEY='<api key here>';\n\n\
                Get API key from https://makersuite.google.com/app/apikey".to_string()
            },
            CliErrorType::ClexLLMInitilizationError(err) =>
                format!("Error initializing ClexLLM: {err}"),
            CliErrorType::ClexLLMGenerationError(err) =>
                format!("Error generating Clex expression: {err}"),
            CliErrorType::InputFormatMissing =>
                "Input format is missing. Please provide the input format using the appropriate flag".to_string(),
            CliErrorType::ConstraintsMissing =>
                "Constraints are missing. Please provide the constraints using the appropriate flag".to_string(),
            CliErrorType::AiRequiredArgsMissing =>
                "Problem URL OR input format and constraints are required. Please provide them using the appropriate flags".to_string(),
            CliErrorType::CScrapperError(err) =>
                format!("Error in cscrapper: {err}"),
        };

        write!(f, "[CLI ERROR] CliErrorType::{self:?} {error_description}")
    }
}

impl Error for CliErrorType {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
