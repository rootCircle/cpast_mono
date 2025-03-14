use reqwest::Client;
use std::{thread, time::Duration};

use crate::{
    CODECHEF_PREFIX, CodePlatform,
    qscrapper::{ProblemScraper, ScrapeAPIResponse, ScraperError},
};

pub(crate) struct CodeChef {
    client: Client,
}

impl CodeChef {
    pub(crate) fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
            .expect("Failed to create HTTP client");
        CodeChef { client }
    }
}
impl ProblemScraper for CodeChef {
    #[allow(clippy::needless_lifetimes)]
    async fn get_problems_by_code<'a>(
        &self,
        platform: &CodePlatform<'a>,
    ) -> Result<ScrapeAPIResponse, ScraperError> {
        let code = match platform {
            CodePlatform::CodeChef(code) => code,
            _ => unreachable!(),
        };
        let url = CODECHEF_PREFIX.replace("{problem_code}", code);
        let response = self.client.get(&url).send().await?;

        thread::sleep(Duration::from_millis(500));

        if response.status().is_success() {
            let json: serde_json::Value = response.json().await?;
            let problem_components = json
                .get("problemComponents")
                .ok_or(ScraperError::ProblemNotFound)?;

            let input_format = problem_components["inputFormat"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let constraints = problem_components["constraints"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let statement = problem_components["statement"]
                .as_str()
                .unwrap_or("")
                .to_string();

            Ok(ScrapeAPIResponse {
                input_format,
                constraints,
                statement,
            })
        } else {
            Err(ScraperError::NetworkError(
                response.error_for_status().unwrap_err(),
            ))
        }
    }
}
