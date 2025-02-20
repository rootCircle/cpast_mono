use actix_web::{ResponseError, http::StatusCode};

pub(crate) mod get;
pub(crate) mod post;

#[derive(thiserror::Error)]
pub enum ShareError {
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

impl std::fmt::Debug for ShareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ShareError {
    fn status_code(&self) -> StatusCode {
        match self {
            ShareError::InvalidClex(_) => StatusCode::BAD_REQUEST,
            ShareError::InvalidShareId(_) => StatusCode::BAD_REQUEST,
            ShareError::DirtyLanguageInDatabase(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ShareError::ShareIdNotFound(_) => StatusCode::NOT_FOUND,
            ShareError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
