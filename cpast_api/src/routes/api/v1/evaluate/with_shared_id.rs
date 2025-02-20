use actix_web::post;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use ccode_runner::lang_runner::runner::LanguageName;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

use super::{EvaluateAPIError, EvaluateCodeResponse};

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
struct EvaluateCodeWithOnlySharedIDRequest {
    #[schema(example = "print('Hello, worldd!')")]
    test_code: String,

    #[schema(example = "python")]
    test_code_language: LanguageName,

    #[schema(example = "1e23fdh-sdf23-23sdf")]
    share_id: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Share_id", body = EvaluateCodeResponse),
        (status = 400, description = "Invalid clex", body = String),
        (status = 500, description = "Internal server error", body = String),
    )
)]
#[post("/with_shared_id")]
pub async fn post_with_shared_id(
    pool: web::Data<PgPool>,
    code_request: Json<EvaluateCodeWithOnlySharedIDRequest>,
) -> Result<HttpResponse, EvaluateAPIError> {
    todo!("Implement post_with_code_and_platform");
}
