use super::ProblemScraper;
use crate::{
    CODEFORCES_PREFIX, CodePlatform,
    qscrapper::{ScrapeAPIResponse, ScraperError},
};
use headless_chrome::{Browser, LaunchOptions};
use reqwest::Client;
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderValue, REFERER, USER_AGENT};
use scraper::{Html, Selector};
use std::ffi::OsStr;
use std::time::Duration;

pub(crate) struct CodeForces {
    client: Client,
}

impl CodeForces {
    pub(crate) fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"
        ));
        headers.insert(ACCEPT, HeaderValue::from_static(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"
        ));
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.5"));
        headers.insert(REFERER, HeaderValue::from_static("https://codeforces.com/"));

        let client = Client::builder()
            .timeout(Duration::from_secs(15))
            .default_headers(headers)
            .cookie_store(true)
            .build()
            .expect("Failed to create HTTP client");

        CodeForces { client }
    }

    // Normal HTTP-based scraping
    async fn scrape_with_http(&self, url: &str) -> Result<ScrapeAPIResponse, ScraperError> {
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            let html = response.text().await?;
            self.parse_problem_html(&html)
        } else {
            Err(ScraperError::NetworkError(
                response.error_for_status().unwrap_err(),
            ))
        }
    }

    // Headless Chrome-based scraping with anti-captcha measures
    async fn scrape_with_headless_chrome(
        &self,
        url: &str,
    ) -> Result<ScrapeAPIResponse, ScraperError> {
        let url = url.to_owned();
        // Spawn a blocking task for the browser operations
        let html = tokio::task::spawn_blocking(move || -> Result<String, String> {
            // Configure browser launch options with anti-detection measures
            let options = LaunchOptions {
                headless: true,
                enable_gpu: false,
                window_size: Some((1920, 1080)),
                sandbox: false,
                args: vec![
                    OsStr::new("--disable-blink-features=AutomationControlled"),
                    OsStr::new("--disable-infobars"),
                    OsStr::new("--start-maximized"),            
                    OsStr::new("--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
                ],
                ..Default::default()
            };

            let browser = Browser::new(options).map_err(|e| format!("Failed to launch browser: {e}"))?;

            let tab = browser.new_tab().map_err(|e| format!("Failed to create new tab: {e}"))?;

            tab.navigate_to(&url)
                .map_err(|e| format!("Failed to navigate to problem page: {e}"))?;

            tab.wait_until_navigated()
                .map_err(|e| format!("Failed to wait for navigation: {e}"))?;

            // Prevent detection by modifying navigator properties
            tab.evaluate(
                r#"Object.defineProperty(navigator, 'webdriver', { get: () => false });"#,
                true,
            ).map_err(|e| format!("Failed to spoof webdriver API: {e}"))?;

            tab.evaluate(r#"
                const originalFetch = window.fetch;
                window.fetch = function(input, init) {
                    const headers = {
                        'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8',
                        'Accept-Language': 'en-US,en;q=0.9',
                        'Cache-Control': 'max-age=0',
                        'Connection': 'keep-alive',
                        'Sec-Ch-Ua': '"Not_A Brand";v="8", "Chromium";v="120"',
                        'Sec-Ch-Ua-Mobile': '?0',
                        'Sec-Ch-Ua-Platform': '"Windows"',
                        'Upgrade-Insecure-Requests': '1',
                        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
                        'Sec-Fetch-Dest': 'document',
                        'Sec-Fetch-Mode': 'navigate',
                        'Sec-Fetch-Site': 'same-origin',
                        'Sec-Fetch-User': '?1'
                    };
                    init = init || {};
                    init.headers = { ...headers, ...init.headers };
                    return originalFetch(input, init);
                };
            "#, true).map_err(|e| format!("Failed to spoof fetch API: {e}"))?;

            tab.evaluate(r#"
                // Fake WebGL vendor & renderer
                const getParameter = WebGLRenderingContext.prototype.getParameter;
                WebGLRenderingContext.prototype.getParameter = function(parameter) {
                    if (parameter === 37445) { // UNMASKED_VENDOR_WEBGL
                        return "Intel Inc.";
                    }
                    if (parameter === 37446) { // UNMASKED_RENDERER_WEBGL
                        return "Intel(R) UHD Graphics 620";
                    }
                    return getParameter(parameter);
                };

                // Fake Canvas Fingerprinting
                HTMLCanvasElement.prototype.toDataURL = function() {
                    return "data:image/png;base64,fake_canvas_fingerprint";
                };

                const getImageData = CanvasRenderingContext2D.prototype.getImageData;
                CanvasRenderingContext2D.prototype.getImageData = function(x, y, width, height) {
                    for (let i = 0; i < arguments.length; i++) {
                        arguments[i] = arguments[i] + Math.random() * 0.00001; // Slightly randomize pixels
                    }
                    return getImageData.apply(this, arguments);
                };
            "#, true).map_err(|e| format!("Failed to spoof canvas and webgl: {e}"))?;


            // Get the HTML content
            let html = tab.get_content()
                .map_err(|e| format!("Failed to get page content: {e}"))?;
            Ok(html)
        }).await.map_err(|e| ScraperError::ParsingError(format!("Task error: {e}")))?
        .map_err(ScraperError::ParsingError)?;

        // Parse the HTML
        self.parse_problem_html(&html)
    }

    // HTML parsing logic extracted to avoid duplication
    fn parse_problem_html(&self, html: &str) -> Result<ScrapeAPIResponse, ScraperError> {
        let document = Html::parse_document(html);

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
            constraints: String::new(),
            statement,
        })
    }
}

impl ProblemScraper for CodeForces {
    #[allow(clippy::needless_lifetimes)]
    async fn get_problems_by_code<'a>(
        &self,
        platform: &CodePlatform<'a>,
    ) -> Result<ScrapeAPIResponse, ScraperError> {
        let (contest_id, code) = match platform {
            CodePlatform::CodeForces(contest_id, code) => (contest_id, code),
            _ => unreachable!(),
        };

        let url = CODEFORCES_PREFIX
            .replace("{contest_id}", contest_id)
            .replace("{problem_code}", code);

        // First try with normal HTTP request
        match self.scrape_with_http(&url).await {
            Ok(response) => Ok(response),
            Err(_) => {
                // If that fails, try with headless Chrome
                match self.scrape_with_headless_chrome(&url).await {
                    Ok(response) => Ok(response),
                    Err(chrome_error) => {
                        // Both methods failed, return the error from the headless Chrome attempt
                        Err(chrome_error)
                    }
                }
            }
        }
    }
}
