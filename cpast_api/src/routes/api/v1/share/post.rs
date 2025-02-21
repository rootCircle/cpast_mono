use actix_web::post;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use anyhow::Context;
use ccode_runner::lang_runner::language_name::LanguageName;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, PgPool, Postgres, Transaction};
use utoipa::ToSchema;
use uuid::{ContextV7, Timestamp, Uuid};

use super::ShareError;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
struct SharePostRequest {
    #[schema(example = "print('Hello, world!')")]
    code: String,

    #[schema(example = "python")]
    language: LanguageName,

    #[schema(example = "N[1,50] S[1, @CH_UPPER@]")]
    clex: String,
}

#[derive(Serialize, ToSchema)]
struct SharePostResponse {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    share_id: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Share_id", body = SharePostResponse),
        (status = 400, description = "Invalid clex", body = String),
        (status = 500, description = "Internal server error", body = String),
    )
)]
#[post("/share")]
pub async fn post_share_code(
    pool: web::Data<PgPool>,
    code_request: Json<SharePostRequest>,
) -> Result<HttpResponse, ShareError> {
    let transaction = pool.begin().await.context("Failed to start transaction")?;
    verify_clex(&code_request.clex)?;

    let share_id = push_code(transaction, code_request.0)
        .await
        .context("Failed to generate share code")?;
    Ok(HttpResponse::Ok().json(SharePostResponse { share_id }))
}

#[tracing::instrument(name = "Push code", skip(transaction, code_request))]
pub(crate) async fn push_code(
    mut transaction: Transaction<'_, Postgres>,
    code_request: SharePostRequest,
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

fn verify_clex(clex: &str) -> Result<(), ShareError> {
    clex::generator(clex.to_string()).map_err(|err| ShareError::InvalidClex(err.to_string()))?;
    Ok(())
}
