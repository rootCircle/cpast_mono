use actix_web::post;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use ccode_runner::lang_runner::language_name::LanguageName;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::routes::api::v1::evaluate::{
    cache_clex_into_db, cache_scrape_into_db, get_cached_scrape_from_db, run_and_compare,
    verify_clex,
};

use super::{EvaluateAPIError, EvaluateCodeResponse};

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
struct EvaluateCodeWithPlatformRequest {
    #[schema(example = "X, Y = map(int, input().split())\nprint(X + Y * 10)")]
    correct_code: String,

    #[schema(example = "python")]
    correct_code_language: LanguageName,

    #[schema(example = "X, Y = map(int, input().split())\nprint(X + Y * 10)")]
    test_code: String,

    #[schema(example = "python")]
    test_code_language: LanguageName,

    #[schema(example = "https://www.codechef.com/problems/WAPEN")]
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
    if code_request.problem_url.is_empty() {
        return Err(EvaluateAPIError::InvalidProblemURL);
    }

    let cached_scrape_clex: Option<String> =
        get_cached_scrape_from_db(&pool, &code_request.problem_url).await?;

    let scrape_clex = match cached_scrape_clex {
        Some(scrape) => scrape,
        None => {
            let code_platform = cscrapper::parse_problem_url(&code_request.problem_url);

            let scrape_clex = match code_platform {
                Some(platform) => {
                    let result = cscrapper::get_problem_statement(platform).await;
                    match result {
                        Ok(response) => response,
                        Err(err) => {
                            return Err(EvaluateAPIError::ScrapperError(err));
                        }
                    }
                }
                None => {
                    return Err(EvaluateAPIError::InvalidProblemURL);
                }
            };

            let clex_llm_generator =
                clex_llm::create_clex_generator(gemini_api_key.expose_secret())
                    .map_err(|e| EvaluateAPIError::ClexLLMError(e.to_string()))?;

            let generated_clex = clex_llm::generate_clex_expression(
                &clex_llm_generator,
                &scrape_clex.input_format,
                &scrape_clex.constraints,
            )
            .await
            .map_err(|e| EvaluateAPIError::ClexLLMError(e.to_string()))?;

            verify_clex(&generated_clex)?;

            // Run both cache operations concurrently
            let (clex_result, scrape_result) = tokio::join!(
                cache_clex_into_db(
                    &pool,
                    &scrape_clex.input_format,
                    &scrape_clex.constraints,
                    &generated_clex,
                ),
                cache_scrape_into_db(
                    &pool,
                    &code_request.problem_url,
                    &scrape_clex.input_format,
                    &scrape_clex.constraints,
                    &scrape_clex.statement,
                    &generated_clex,
                )
            );

            // Check results of both operations
            clex_result?;
            scrape_result?;

            generated_clex
        }
    };

    let response = run_and_compare(
        &code_request.correct_code,
        &code_request.test_code,
        code_request.correct_code_language.clone(),
        code_request.test_code_language.clone(),
        &scrape_clex,
    )?;

    Ok(HttpResponse::Ok().json(response))
}
