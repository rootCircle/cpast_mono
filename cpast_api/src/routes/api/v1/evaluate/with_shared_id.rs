use actix_web::post;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use anyhow::Context;
use ccode_runner::lang_runner::language_name::LanguageName;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::{Uuid, Version};

use crate::routes::api::v1::evaluate::run_and_compare;

use super::{EvaluateAPIError, EvaluateCodeResponse};

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
struct EvaluateCodeWithOnlySharedIDRequest {
    #[schema(example = "print('Hello, worldd!')")]
    test_code: String,

    #[schema(example = "python")]
    test_code_language: LanguageName,

    #[schema(example = "01959403-d3e4-7752-85bd-a304b561692d")]
    share_id: String,
}

#[derive(Serialize, ToSchema)]
struct ShareGetResponse {
    #[schema(example = "print('Hello, world!')")]
    code: String,

    #[schema(example = "python")]
    language: LanguageName,

    #[schema(example = "N[1,50] S[1, 10, @CH_UPPER@]")]
    clex: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successful evaluation", body = EvaluateCodeResponse),
        (status = 400, description = "Invalid share ID or format", body = String),
        (status = 404, description = "Share ID not found", body = String), 
        (status = 500, description = "Server error - includes database errors or language parsing errors", body = String)
    )
)]
#[post("/with_shared_id")]
pub async fn post_with_shared_id(
    pool: web::Data<PgPool>,
    code_request: Json<EvaluateCodeWithOnlySharedIDRequest>,
) -> Result<HttpResponse, EvaluateAPIError> {
    let parsed_share_id = Uuid::parse_str(&code_request.share_id)
        .map_err(|err| EvaluateAPIError::InvalidShareId(err.to_string()))?;
    if parsed_share_id.get_version() != Some(Version::SortRand) {
        return Err(EvaluateAPIError::InvalidShareId(
            "Invalid share id version".to_string(),
        ));
    }
    let correct_code_in_shared_db = get_code_from_share_id(&pool, &code_request.share_id)
        .await
        .map_err(|err| EvaluateAPIError::ShareIdNotFound(err.to_string()))?
        .ok_or(EvaluateAPIError::ShareIdNotFound(
            "Share ID not found".to_string(),
        ))?;

    let response = run_and_compare(
        &correct_code_in_shared_db.code,
        &code_request.test_code,
        correct_code_in_shared_db.language,
        code_request.test_code_language.clone(),
        &correct_code_in_shared_db.clex,
    )?;

    Ok(HttpResponse::Ok().json(response))
}

#[tracing::instrument(name = "Get code from share id", skip(pool))]
pub(crate) async fn get_code_from_share_id(
    pool: &PgPool,
    share_id: &str,
) -> Result<Option<ShareGetResponse>, anyhow::Error> {
    let query = sqlx::query!(
        r#"
        SELECT code, code_language AS "language", clex
        FROM shared_code
        WHERE share_id = $1
        LIMIT 1;
        "#,
        share_id,
    );

    let code_details = query
        .fetch_optional(pool)
        .await
        .context("Failed to fetch code details")?;

    Ok(match code_details {
        Some(details) => Some(ShareGetResponse {
            code: details.code,
            language: details
                .language
                .try_into()
                .map_err(EvaluateAPIError::DirtyLanguageInDatabase)?,
            clex: details.clex,
        }),
        None => None,
    })
}
