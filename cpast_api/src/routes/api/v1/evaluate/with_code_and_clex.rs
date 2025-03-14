use actix_web::HttpResponse;
use actix_web::post;
use actix_web::web::Json;
use ccode_runner::lang_runner::language_name::LanguageName;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::routes::api::v1::evaluate::run_and_compare;
use crate::routes::api::v1::evaluate::verify_clex;

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

    #[schema(example = "N[1,50] S[1, 10, @CH_UPPER@]")]
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
    code_request: Json<EvaluateCodeWithClexRequest>,
) -> Result<HttpResponse, EvaluateAPIError> {
    verify_clex(&code_request.clex)?;

    let response = run_and_compare(
        &code_request.correct_code,
        &code_request.test_code,
        code_request.correct_code_language.clone(),
        code_request.test_code_language.clone(),
        &code_request.clex,
    )?;

    Ok(HttpResponse::Ok().json(response))
}
