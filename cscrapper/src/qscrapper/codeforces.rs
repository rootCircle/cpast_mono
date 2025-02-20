use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::time::Duration;

use crate::{
    CODEFORCES_PREFIX, CodePlatform,
    qscrapper::{ScrapeAPIResponse, ScraperError},
};

use super::ProblemScraper;

pub(crate) struct CodeForces {
    client: Client,
}

impl CodeForces {
    pub(crate) fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
            .expect("Failed to create HTTP client");
        CodeForces { client }
    }
}

impl ProblemScraper for CodeForces {
    fn get_problems_by_code(
        &self,
        platform: &CodePlatform,
    ) -> Result<ScrapeAPIResponse, ScraperError> {
        let (contest_id, code) = match platform {
            CodePlatform::CodeForces(contest_id, code) => (contest_id, code),
            _ => unreachable!(),
        };
        let url = CODEFORCES_PREFIX
            .replace("{contest_id}", contest_id)
            .replace("{problem_code}", code);
        let response = self.client.get(&url).send()?;

        if response.status().is_success() {
            let html = response.text()?;
            eprintln!("{}", html);
            let document = Html::parse_document(&html);

            let problem_statement = match Selector::parse("div.problem-statement") {
                Ok(selector) => selector,
                Err(_) => {
                    return Err(ScraperError::ParsingError(
                        "Can't get the problem statement from the website".to_string(),
                    ));
                }
            };
            let problem_components =
                document
                    .select(&problem_statement)
                    .next()
                    .ok_or(ScraperError::ParsingError(
                        "Can't get the problem statement from the website".to_string(),
                    ))?;

            let input_spec = match Selector::parse("div.input-specification") {
                Ok(selector) => selector,
                Err(_) => {
                    return Err(ScraperError::ParsingError(
                        "Can't get the input specification from the website".to_string(),
                    ));
                }
            };

            let input_format = problem_components
                .select(&input_spec)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            let statement_selector =
                match Selector::parse("div[class='problem-statement'] > div:not([class])") {
                    Ok(selector) => selector,
                    Err(_) => {
                        return Err(ScraperError::ParsingError(
                            "Can't get the problem statement from the website".to_string(),
                        ));
                    }
                };

            let statement = problem_components
                .select(&statement_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            Ok(ScrapeAPIResponse {
                input_format,
                constraints: String::new(), // CodeForces doesn't have a separate constraints section
                statement,
            })
        } else {
            Err(ScraperError::NetworkError(
                response.error_for_status().unwrap_err(),
            ))
        }
    }
}
