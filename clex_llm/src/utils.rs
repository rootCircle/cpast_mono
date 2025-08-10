use google_generative_ai_rs::v1::{
    api::{Client, PostResult},
    errors::GoogleAPIError,
    gemini::request::Request,
};
use reqwest::StatusCode;
use std::time::Duration;

/// Configuration for retry behavior when posting to the Gemini API.
pub struct RetryConfig {
    /// Maximum number of retry attempts on retryable errors.
    pub max_retries: usize,
    /// Initial backoff duration before the first retry.
    pub initial_backoff: Duration,
    /// Maximum backoff duration.
    pub max_backoff: Duration,
    /// Maximum random jitter (in milliseconds) added to the backoff.
    pub jitter_max_ms: u64,
    /// HTTP status codes considered retryable.
    pub retryable_statuses: &'static [StatusCode],
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 5,
            initial_backoff: Duration::from_secs(2),
            max_backoff: Duration::from_secs(30),
            jitter_max_ms: 250,
            retryable_statuses: &[
                StatusCode::TOO_MANY_REQUESTS,
                StatusCode::INTERNAL_SERVER_ERROR,
                StatusCode::BAD_GATEWAY,
                StatusCode::SERVICE_UNAVAILABLE,
                StatusCode::GATEWAY_TIMEOUT,
                StatusCode::REQUEST_TIMEOUT,
            ],
        }
    }
}

/// Post a Gemini request with retry logic using the provided configuration.
pub(crate) async fn post_with_retry_config(
    client: &Client,
    request_timeout_secs: u64,
    request: &Request,
    cfg: &RetryConfig,
) -> Result<PostResult, GoogleAPIError> {
    let mut backoff = cfg.initial_backoff;

    let result = {
        let mut attempt = 0;
        loop {
            match client.post(request_timeout_secs, request).await {
                Ok(res) => break res,
                Err(err) => {
                    let retryable = match err.code {
                        Some(code) => cfg.retryable_statuses.contains(&code),
                        None => false,
                    };

                    if !retryable || attempt >= cfg.max_retries {
                        return Err(err);
                    }

                    // Exponential backoff with jitter.
                    let jitter_ms = if cfg.jitter_max_ms == 0 {
                        0
                    } else {
                        (std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .subsec_nanos() as u64)
                            % cfg.jitter_max_ms
                    };

                    tokio::time::sleep(backoff + Duration::from_millis(jitter_ms)).await;
                    backoff = std::cmp::min(backoff + backoff, cfg.max_backoff);
                    attempt += 1;
                }
            }
        }
    };

    Ok(result)
}

/// Post a Gemini request with default retry behavior.
///
/// - Retries on 429/5xx and request timeout errors up to 3 times.
/// - Initial backoff 2s, doubles each retry up to 30s, with 0-250ms jitter.
pub(crate) async fn post_with_retry(
    client: &Client,
    request_timeout_secs: u64,
    request: &Request,
) -> Result<PostResult, GoogleAPIError> {
    let cfg = RetryConfig::default();
    post_with_retry_config(client, request_timeout_secs, request, &cfg).await
}
