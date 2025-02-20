use crate::qscrapper::ProblemScraper;
use qscrapper::{ScrapeAPIResponse, ScraperError, codechef::CodeChef, codeforces::CodeForces};
mod qscrapper;

const CODECHEF_PREFIX: &str =
    "https://www.codechef.com/api/contests/PRACTICE/problems/{problem_code}";
const CODEFORCES_PREFIX: &str =
    "https://m1.codeforces.com/contest/{contest_id}/problem/{problem_code}";

pub enum CodePlatform<'a> {
    /// CodeChef platform (code)
    CodeChef(&'a str),

    /// CodeForces platform (contest_id, code)
    CodeForces(&'a str, &'a str),
}

pub fn get_problem_statement(platform: CodePlatform) -> Result<ScrapeAPIResponse, ScraperError> {
    match platform {
        CodePlatform::CodeChef(_) => {
            let scraper = CodeChef::new();
            scraper.get_problems_by_code(&platform)
        }
        CodePlatform::CodeForces(_, _) => {
            let scraper = CodeForces::new();
            scraper.get_problems_by_code(&platform)
        }
    }
}
