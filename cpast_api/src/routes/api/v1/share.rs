use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{post, ResponseError};
use actix_web::{web, HttpResponse};
use anyhow::Context;
use ccode_runner::lang_runner::runner::LanguageName;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, PgPool, Postgres, Transaction};
use utoipa::ToSchema;
use uuid::{ContextV7, Timestamp, Uuid};

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
struct CodeRequest {
    #[schema(example = "print('Hello, world!')")]
    code: String,

    #[schema(example = "python")]
    language: LanguageName,

    #[schema(example = "N[1,50] S[1, @CH_UPPER@]")]
    clex: String,
}

#[derive(Serialize, ToSchema)]
struct CodeResponse {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    share_id: String,
}

#[derive(thiserror::Error)]
pub enum ShareError {
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
            ShareError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Share_id", body = CodeResponse),
        (status = 500, description = "Internal server error", body = String),
    )
)]
#[post("/share")]
pub async fn share_code(
    pool: web::Data<PgPool>,
    code_request: Json<CodeRequest>,
) -> Result<HttpResponse, ShareError> {
    let transaction = pool.begin().await.context("Failed to start transaction")?;
    let share_id = push_code(transaction, code_request.0)
        .await
        .context("Failed to generate share code")?;
    Ok(HttpResponse::Ok().json(CodeResponse { share_id }))
}

#[tracing::instrument(name = "Push code", skip(transaction, code_request))]
pub(crate) async fn push_code(
    mut transaction: Transaction<'_, Postgres>,
    code_request: CodeRequest,
) -> Result<String, anyhow::Error> {
    let context = ContextV7::new();
    let share_id = Uuid::new_v7(Timestamp::now(&context)).to_string();
    let query = sqlx::query!(
        r#"
        INSERT INTO shared_code (share_id, code, code_language, clex)
        VALUES ($1, $2, $3, $4)
        "#,
        share_id,
        code_request.code,
        code_request.language.to_string(),
        code_request.clex
    );

    transaction.execute(query).await?;

    transaction.commit().await?;

    Ok(share_id)
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
