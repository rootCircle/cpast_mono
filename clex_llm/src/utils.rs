#![allow(unused)]

use reqwest::StatusCode;
use rig::completion::CompletionError;
use std::future::Future;
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

/// Determine if a reqwest HTTP error should be retried based on status or type.
fn is_retryable_http_error(err: &reqwest::Error, cfg: &RetryConfig) -> bool {
    if let Some(status) = err.status()
        && cfg.retryable_statuses.contains(&status)
    {
        return true;
    }
    // Network hiccups/timeouts without a status
    err.is_timeout() || err.is_connect()
}

/// Check if a rig completion error is retryable using HTTP context.
fn is_retryable_completion_error(err: &CompletionError, cfg: &RetryConfig) -> bool {
    match err {
        CompletionError::HttpError(http) => is_retryable_http_error(http, cfg),
        _ => false,
    }
}

/// Generic retry helper for async operations that return `Result<T, CompletionError>`.
/// Retries on HTTP-retryable errors based on the provided RetryConfig.
pub(crate) async fn retry_completion<F, Fut, T>(
    mut op: F,
    cfg: &RetryConfig,
) -> Result<T, CompletionError>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, CompletionError>>,
{
    let mut backoff = cfg.initial_backoff;
    let mut attempt = 0usize;

    loop {
        match op().await {
            Ok(val) => return Ok(val),
            Err(err) => {
                if !is_retryable_completion_error(&err, cfg) || attempt >= cfg.max_retries {
                    return Err(err);
                }

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
                backoff = std::cmp::min(backoff.saturating_mul(2), cfg.max_backoff);
                attempt += 1;
            }
        }
    }
}
