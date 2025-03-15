use crate::qscrapper::ProblemScraper;
use qscrapper::{ScrapeAPIResponse, ScraperError, codechef::CodeChef, codeforces::CodeForces};
use regex::Regex;
pub mod qscrapper;

const CODECHEF_PREFIX: &str =
    "https://www.codechef.com/api/contests/PRACTICE/problems/{problem_code}";
const CODEFORCES_PREFIX: &str =
    "https://mirror.codeforces.com/contest/{contest_id}/problem/{problem_code}";

#[derive(Debug, PartialEq)]
pub enum CodePlatform<'a> {
    /// CodeChef platform (code)
    CodeChef(&'a str),

    /// CodeForces platform (contest_id, code)
    CodeForces(&'a str, &'a str),
}

#[allow(clippy::needless_lifetimes)]
pub async fn get_problem_statement<'a>(
    platform: CodePlatform<'a>,
) -> Result<ScrapeAPIResponse, ScraperError> {
    match platform {
        CodePlatform::CodeChef(_) => {
            let scraper = CodeChef::new();
            scraper.get_problems_by_code(&platform).await
        }
        CodePlatform::CodeForces(_, _) => {
            let scraper = CodeForces::new();
            scraper.get_problems_by_code(&platform).await
        }
    }
}

pub fn parse_problem_url(url: &str) -> Option<CodePlatform> {
    let codechef_re = Regex::new(r"(?:https?://)?(?:www\.)?codechef\.com/(?:[A-Z0-9]+/problems|problems|practice/course/[^/]+/[^/]+/problems)/([a-zA-Z0-9_]+)").ok()?;
    let codeforces_re = Regex::new(
        r"(?:https?://)?(?:www\.)?codeforces\.com/(?:contest|gym)/([0-9]+)/problem/([A-Z][0-9]?)",
    )
    .ok()?;
    let codeforces_problemset_re = Regex::new(
        r"(?:https?://)?(?:www\.)?codeforces\.com/problemset/problem/([0-9]+)/([A-Z][0-9]?)",
    )
    .ok()?;

    if let Some(captures) = codechef_re.captures(url) {
        return Some(CodePlatform::CodeChef(captures.get(1)?.as_str()));
    }

    if let Some(captures) = codeforces_re.captures(url) {
        return Some(CodePlatform::CodeForces(
            captures.get(1)?.as_str(),
            captures.get(2)?.as_str(),
        ));
    }

    if let Some(captures) = codeforces_problemset_re.captures(url) {
        return Some(CodePlatform::CodeForces(
            captures.get(1)?.as_str(),
            captures.get(2)?.as_str(),
        ));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codechef_urls() {
        assert_eq!(
            parse_problem_url("https://www.codechef.com/START146C/problems/BOUNCE_BALL"),
            Some(CodePlatform::CodeChef("BOUNCE_BALL"))
        );
        assert_eq!(
            parse_problem_url("https://www.codechef.com/problems/NOWINNER"),
            Some(CodePlatform::CodeChef("NOWINNER"))
        );
        assert_eq!(
            parse_problem_url(
                "https://www.codechef.com/practice/course/arrays/ARRAYS/problems/SEARCHINARR"
            ),
            Some(CodePlatform::CodeChef("SEARCHINARR"))
        );

        assert_eq!(
            parse_problem_url("www.codechef.com/problems/NOWINNER"),
            Some(CodePlatform::CodeChef("NOWINNER"))
        );
        assert_eq!(
            parse_problem_url("codechef.com/problems/NOWINNER"),
            Some(CodePlatform::CodeChef("NOWINNER"))
        );
        assert_eq!(
            parse_problem_url("http://codechef.com/problems/NOWINNER"),
            Some(CodePlatform::CodeChef("NOWINNER"))
        );
        assert_eq!(
            parse_problem_url("http://www.codechef.com/problems/NOWINNER"),
            Some(CodePlatform::CodeChef("NOWINNER"))
        );
    }

    #[test]
    fn test_codeforces_urls() {
        assert_eq!(
            parse_problem_url("https://codeforces.com/contest/1234/problem/A"),
            Some(CodePlatform::CodeForces("1234", "A"))
        );
        assert_eq!(
            parse_problem_url("https://codeforces.com/gym/105783/problem/B"),
            Some(CodePlatform::CodeForces("105783", "B"))
        );
        assert_eq!(
            parse_problem_url("codeforces.com/gym/105783/problem/B"),
            Some(CodePlatform::CodeForces("105783", "B"))
        );
        assert_eq!(
            parse_problem_url("http://codeforces.com/gym/105783/problem/B"),
            Some(CodePlatform::CodeForces("105783", "B"))
        );
        assert_eq!(
            parse_problem_url("http://www.codeforces.com/gym/105783/problem/B"),
            Some(CodePlatform::CodeForces("105783", "B"))
        );
        assert_eq!(
            parse_problem_url("www.codeforces.com/gym/105783/problem/B"),
            Some(CodePlatform::CodeForces("105783", "B"))
        );
        assert_eq!(
            parse_problem_url("https://www.codeforces.com/gym/105783/problem/B"),
            Some(CodePlatform::CodeForces("105783", "B"))
        );
        assert_eq!(
            parse_problem_url("https://codeforces.com/problemset/problem/1234/A"),
            Some(CodePlatform::CodeForces("1234", "A"))
        );
    }

    #[test]
    fn test_invalid_urls() {
        assert_eq!(
            parse_problem_url("https://example.com/contest/1234/problem/A"),
            None
        );
        assert_eq!(
            parse_problem_url("https://codechef.com/some/invalid/url"),
            None
        );
        assert_eq!(
            parse_problem_url("https://codeforces.com/contest/abcd/problem/X"),
            None
        );
    }
}
