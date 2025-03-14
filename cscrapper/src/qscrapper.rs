pub(crate) mod codechef;
pub(crate) mod codeforces;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::CodePlatform;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScrapeAPIResponse {
    pub input_format: String,
    pub constraints: String,
    pub statement: String,
}

#[derive(Error, Debug)]
pub enum ScraperError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Problem not found")]
    ProblemNotFound,
    #[error("Parsing error: {0}")]
    ParsingError(String),
}

pub trait ProblemScraper {
    fn get_problems_by_code(
        &self,
        platform: &CodePlatform,
    ) -> impl std::future::Future<Output = Result<ScrapeAPIResponse, ScraperError>> + Send;
}
