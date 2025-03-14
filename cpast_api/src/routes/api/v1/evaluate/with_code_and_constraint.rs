use actix_web::post;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use anyhow::Context;
use ccode_runner::lang_runner::language_name::LanguageName;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, PgPool, Postgres, Transaction};
use utoipa::ToSchema;

use crate::routes::api::v1::evaluate::{
    hash_input_format_and_constraints, run_and_compare, verify_clex,
};

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

    #[schema(example = "One integer")]
    input_format: String,

    #[schema(example = "1 <= Integer < 50000")]
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
    gemini_api_key: web::Data<SecretString>,
    code_request: Json<EvaluateCodeWithConstraintRequest>,
) -> Result<HttpResponse, EvaluateAPIError> {
    if code_request.input_format.is_empty() || code_request.constraints.is_empty() {
        return Err(EvaluateAPIError::InvalidInputFormatOrConstraints);
    }

    let cached_clex =
        get_cached_clex_from_db(&pool, &code_request.input_format, &code_request.constraints)
            .await?;

    let clex = match cached_clex {
        Some(cached_clex) => {
            verify_clex(&cached_clex)?;
            cached_clex
        }
        None => {
            let clex_llm_generator = clex_llm::create_generator(gemini_api_key.expose_secret())
                .map_err(|e| EvaluateAPIError::ClexLLMError(e.to_string()))?;

            let generated_clex = clex_llm::generate_clex_expression(
                &clex_llm_generator,
                &code_request.input_format,
                &code_request.constraints,
            )
            .await
            .map_err(|e| EvaluateAPIError::ClexLLMError(e.to_string()))?;

            verify_clex(&generated_clex)?;

            cache_clex_into_db(
                &pool,
                &code_request.input_format,
                &code_request.constraints,
                &generated_clex,
            )
            .await?;
            generated_clex
        }
    };

    let response = run_and_compare(
        &code_request.correct_code,
        &code_request.test_code,
        code_request.correct_code_language.clone(),
        code_request.test_code_language.clone(),
        &clex,
    )?;

    Ok(HttpResponse::Ok().json(response))
}

#[tracing::instrument(name = "Try getting cached clex from DB", skip(pool))]
pub(crate) async fn get_cached_clex_from_db(
    pool: &PgPool,
    input_format: &str,
    constraints: &str,
) -> Result<Option<String>, anyhow::Error> {
    let query = sqlx::query!(
        r#"
        SELECT clex
        FROM llm_cache
        WHERE input_hash = $1 
        AND created_at + ttl > NOW()
        LIMIT 1;
        "#,
        hash_input_format_and_constraints(input_format, constraints)
    );

    let result = query
        .fetch_optional(pool)
        .await
        .context("Failed to fetch code details")?;

    Ok(result.map(|row| row.clex))
}

pub(crate) async fn cache_clex_into_db(
    pool: &PgPool,
    input_format: &str,
    constraints: &str,
    clex: &str,
) -> Result<(), anyhow::Error> {
    let mut transaction: Transaction<'_, Postgres> =
        pool.begin().await.context("Failed to start transaction")?;

    let query = sqlx::query!(
        r#"
        INSERT INTO llm_cache (input_hash, input_format, constraints, clex)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (input_hash) DO UPDATE SET clex = $2;
        "#,
        hash_input_format_and_constraints(input_format, constraints),
        input_format,
        constraints,
        clex
    );

    transaction.execute(query).await?;

    transaction.commit().await?;

    Ok(())
}
