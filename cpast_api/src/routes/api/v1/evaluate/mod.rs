#![allow(dead_code, unused_variables)]
use actix_web::{http::StatusCode, ResponseError};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};

pub(crate) mod with_code_and_clex;
pub(crate) mod with_code_and_constraint;
pub(crate) mod with_code_and_platform;
pub(crate) mod with_platform;
pub(crate) mod with_shared_id;

#[derive(OpenApi)]
#[openapi(paths(
    crate::routes::api::v1::evaluate::with_code_and_constraint::post_with_code_and_constraint,
    crate::routes::api::v1::evaluate::with_code_and_clex::post_with_code_and_clex,
    crate::routes::api::v1::evaluate::with_code_and_platform::post_with_code_and_platform,
    crate::routes::api::v1::evaluate::with_shared_id::post_with_shared_id,
    crate::routes::api::v1::evaluate::with_platform::post_with_platform,
))]
pub(crate) struct EvaluateCodeApiv1;

#[derive(Serialize, ToSchema)]
struct EvaluateCodeInputDiff {
    #[schema(example = "world")]
    input: String,

    #[schema(example = "Hello, world!")]
    expected_output: String,

    #[schema(example = "Hello, worldd!")]
    actual_output: String,
}

#[derive(Serialize, ToSchema)]
struct EvaluateCodeResponse {
    #[schema(example = false)]
    has_output_matched: bool,

    #[schema(example = json!(Vec::from([EvaluateCodeInputDiff {
        input: "world".to_string(),
        expected_output: "Hello, world!".to_string(),
        actual_output: "Hello, worldd!".to_string(),
    }])))]
    input_diffs: Vec<EvaluateCodeInputDiff>,
}

#[derive(thiserror::Error)]
pub enum EvaluateAPIError {
    #[error("{0}")]
    InvalidClex(String),

    #[error("{0}")]
    DirtyLanguageInDatabase(String),

    #[error("{0}")]
    ShareIdNotFound(String),

    #[error("{0}")]
    InvalidShareId(String),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for EvaluateAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for EvaluateAPIError {
    fn status_code(&self) -> StatusCode {
        match self {
            EvaluateAPIError::InvalidClex(_) => StatusCode::BAD_REQUEST,
            EvaluateAPIError::InvalidShareId(_) => StatusCode::BAD_REQUEST,
            EvaluateAPIError::DirtyLanguageInDatabase(_) => StatusCode::INTERNAL_SERVER_ERROR,
            EvaluateAPIError::ShareIdNotFound(_) => StatusCode::NOT_FOUND,
            EvaluateAPIError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
