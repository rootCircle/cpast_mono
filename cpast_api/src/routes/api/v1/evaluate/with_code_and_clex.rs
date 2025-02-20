use actix_web::post;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use ccode_runner::lang_runner::runner::LanguageName;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

use super::{EvaluateAPIError, EvaluateCodeResponse};

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
struct EvaluateCodeWithClexRequest {
    #[schema(example = "print('Hello, world!')")]
    correct_code: String,

    #[schema(example = "python")]
    correct_code_language: LanguageName,

    #[schema(example = "print('Hello, worldd!')")]
    test_code: String,

    #[schema(example = "python")]
    test_code_language: LanguageName,

    #[schema(example = "N[1,50] S[1, @CH_UPPER@]")]
    clex: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Share_id", body = EvaluateCodeResponse),
        (status = 400, description = "Invalid clex", body = String),
        (status = 500, description = "Internal server error", body = String),
    )
)]
#[post("/with_code_and_clex")]
pub async fn post_with_code_and_clex(
    pool: web::Data<PgPool>,
    code_request: Json<EvaluateCodeWithClexRequest>,
) -> Result<HttpResponse, EvaluateAPIError> {
    todo!("Implement post_with_code_and_clex");
}
