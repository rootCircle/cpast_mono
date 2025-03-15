use reqwest::Client;
use std::time::Duration;

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
            .timeout(Duration::from_secs(10))
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
            let mut statement = problem_components["statement"]
                .as_str()
                .unwrap_or("")
                .to_string();

            let output_format = problem_components["outputFormat"]
                .as_str()
                .unwrap_or("")
                .to_string();
            statement.push_str("\n\nOutput:\n");
            statement.push_str(&output_format);

            let empty_vec = vec![];
            let sample_test_cases = problem_components["sampleTestcases"]
                .as_array()
                .unwrap_or(&empty_vec);

            if !sample_test_cases.is_empty() {
                statement.push_str("\n\nSample Test Cases:\n\n");
            }

            for test_case in sample_test_cases {
                let input = test_case["input"].as_str().unwrap_or("");
                let output = test_case["output"].as_str().unwrap_or("");
                let explanation = test_case["explanation"].as_str().unwrap_or("");
                let is_deleted = test_case["isDeleted"].as_bool().unwrap_or(false);

                if is_deleted {
                    continue;
                }

                statement.push_str("Example Input:\n");
                statement.push_str(input);
                statement.push_str("\nExample Output:\n");
                statement.push_str(output);
                statement.push_str("\nExplanation:\n\n");
                statement.push_str(explanation);
            }

            let user_tags = json.get("user_tags").unwrap_or(&serde_json::Value::Null);
            if !user_tags.is_null() {
                statement.push_str("\n\nUser Tags:\n");
                for tag in user_tags.as_array().unwrap_or(&vec![]) {
                    statement.push_str(tag.as_str().unwrap_or(""));
                    statement.push_str(", ");
                }
            }

            let computed_tags = json
                .get("computed_tags")
                .unwrap_or(&serde_json::Value::Null);
            if !computed_tags.is_null() {
                statement.push_str("\n\nComputed Tags:\n");
                for tag in computed_tags.as_array().unwrap_or(&vec![]) {
                    statement.push_str(tag.as_str().unwrap_or(""));
                    statement.push_str(", ");
                }
            }

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
