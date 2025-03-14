use actix_web::post;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use ccode_runner::lang_runner::language_name::LanguageName;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

use super::{EvaluateAPIError, EvaluateCodeResponse};

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
struct EvaluateCodeWithPlatformRequest {
    #[schema(example = "print('Hello,', input(), '!')")]
    correct_code: String,

    #[schema(example = "python")]
    correct_code_language: LanguageName,

    #[schema(example = "print('Hello, worldd!')")]
    test_code: String,

    #[schema(example = "python")]
    test_code_language: LanguageName,

    // TODO: Use custom in house URL type instead of String
    #[schema(example = "https://codeforces.com/problemset/problem/4/A")]
    problem_url: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Share_id", body = EvaluateCodeResponse),
        (status = 400, description = "Invalid clex", body = String),
        (status = 500, description = "Internal server error", body = String),
    )
)]
#[post("/with_code_and_platform")]
pub async fn post_with_code_and_platform(
    pool: web::Data<PgPool>,
    gemini_api_key: web::Data<SecretString>,
    code_request: Json<EvaluateCodeWithPlatformRequest>,
) -> Result<HttpResponse, EvaluateAPIError> {
    todo!("Implement post_with_code_and_platform");
}
