use actix_web::post;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use ccode_runner::lang_runner::runner::LanguageName;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

use super::{EvaluateAPIError, EvaluateCodeResponse};

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
struct EvaluateCodeWithConstraintRequest {
    #[schema(example = "print('Hello,', input(), '!')")]
    correct_code: String,

    #[schema(example = "python")]
    correct_code_language: LanguageName,

    #[schema(example = "print('Hello, worldd!')")]
    test_code: String,

    #[schema(example = "python")]
    test_code_language: LanguageName,

    #[schema(example = "Print hello followed by inputted name")]
    problem_description: String,

    #[schema(example = "One string input")]
    input_format: String,

    #[schema(example = "length of name is less than 50")]
    constraints: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Share_id", body = EvaluateCodeResponse),
        (status = 400, description = "Invalid clex", body = String),
        (status = 500, description = "Internal server error", body = String),
    )
)]
#[post("/with_code_and_constraint")]
pub async fn post_with_code_and_constraint(
    pool: web::Data<PgPool>,
    code_request: Json<EvaluateCodeWithConstraintRequest>,
) -> Result<HttpResponse, EvaluateAPIError> {
    todo!("Implement post_with_code_and_clex");
}
