use crate::routes::api::v1::share::ShareError;
use actix_web::{HttpResponse, get, web};
use anyhow::Context;
use ccode_runner::lang_runner::language_name::LanguageName;
use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::{Uuid, Version};

#[derive(Serialize, ToSchema)]
struct ShareGetResponse {
    #[schema(example = "print('Hello, world!')")]
    code: String,

    #[schema(example = "python")]
    language: LanguageName,

    #[schema(example = "N[1,50] S[1, @CH_UPPER@]")]
    clex: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Code details", body = ShareGetResponse),
        (status = 400, description = "Invalid input", body = String),
        (status = 404, description = "Share ID not found", body = String),
        (status = 500, description = "Internal server error", body = String),
    )
)]
#[tracing::instrument(name = "get_code", skip(pool))]
#[get("/share/{share_id}")]
pub async fn get_share_code(
    pool: web::Data<PgPool>,
    share_id: web::Path<String>,
) -> Result<HttpResponse, ShareError> {
    let parsed_share_id = Uuid::parse_str(share_id.as_str())
        .map_err(|err| ShareError::InvalidShareId(err.to_string()))?;
    if parsed_share_id.get_version() != Some(Version::SortRand) {
        return Err(ShareError::InvalidShareId(
            "Invalid share id version".to_string(),
        ));
    }
    let code_details_build = get_code_from_share_id(&pool, &share_id)
        .await
        .map_err(|err| ShareError::ShareIdNotFound(err.to_string()))?;
    Ok(HttpResponse::Ok().json(code_details_build))
}

#[tracing::instrument(name = "Get code from share id", skip(pool))]
pub(crate) async fn get_code_from_share_id(
    pool: &PgPool,
    share_id: &str,
) -> Result<ShareGetResponse, anyhow::Error> {
    let query = sqlx::query!(
        r#"
        SELECT code, code_language AS "language", clex
        FROM shared_code
        WHERE share_id = $1
        "#,
        share_id,
    );

    let code_details = query
        .fetch_one(pool)
        .await
        .context("Failed to fetch code details")?;

    Ok(ShareGetResponse {
        code: code_details.code,
        language: code_details
            .language
            .try_into()
            .map_err(ShareError::DirtyLanguageInDatabase)?,
        clex: code_details.clex,
    })
}
