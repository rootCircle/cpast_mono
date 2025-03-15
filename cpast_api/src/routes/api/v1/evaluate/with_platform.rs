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
    cache_clex_into_db, cache_scrape_into_db, run_and_compare, verify_clex,
};

use super::{EvaluateAPIError, EvaluateCodeResponse};

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
struct EvaluateCodeWithOnlyPlatformRequest {
    #[schema(example = "X, Y = map(int, input().split())\nprint(X + Y * 10)")]
    test_code: String,

    #[schema(example = "python")]
    test_code_language: LanguageName,

    #[schema(example = "https://www.codechef.com/problems/WAPEN")]
    problem_url: String,
}

#[derive(Debug)]
struct ScrapeCacheStoreResponse {
    statement: String,
    input_format: String,
    constraints: String,
    clex: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Share_id", body = EvaluateCodeResponse),
        (status = 400, description = "Invalid clex", body = String),
        (status = 500, description = "Internal server error", body = String),
    )
)]
#[post("/with_platform")]
pub async fn post_with_platform(
    pool: web::Data<PgPool>,
    gemini_api_key: web::Data<SecretString>,
    code_request: Json<EvaluateCodeWithOnlyPlatformRequest>,
) -> Result<HttpResponse, EvaluateAPIError> {
    if code_request.problem_url.is_empty() {
        return Err(EvaluateAPIError::InvalidProblemURL);
    }

    let (cached_scrape_store, cached_generated_code) = tokio::join!(
        get_cached_all_data_scrape_from_db(&pool, &code_request.problem_url),
        get_cached_code_gen_llm(&pool, &code_request.problem_url)
    );

    let cached_generated_code = cached_generated_code?;
    let cached_scrape_store = cached_scrape_store?;

    let (correct_code_llm, lang_name, clex) = match cached_scrape_store {
        Some(scrape_store) => match cached_generated_code {
            Some((cached_code, code_language)) => (cached_code, code_language, scrape_store.clex),
            None => {
                let clex_llm_generator =
                    clex_llm::create_code_generator(gemini_api_key.expose_secret())
                        .map_err(|e| EvaluateAPIError::ClexLLMError(e.to_string()))?;

                let (generated_code, generated_lang) = clex_llm::generate_code_solution(
                    &clex_llm_generator,
                    &scrape_store.statement,
                    &scrape_store.input_format,
                    &scrape_store.constraints,
                )
                .await
                .map_err(|e| EvaluateAPIError::ClexLLMError(e.to_string()))?;

                cache_code_gen_llm_into_db(
                    &pool,
                    &code_request.problem_url,
                    &generated_code,
                    generated_lang.clone(),
                )
                .await
                .map_err(|e| EvaluateAPIError::ClexLLMError(e.to_string()))?;

                (generated_code, generated_lang, scrape_store.clex)
            }
        },
        None => {
            match cached_generated_code {
                Some((cached_code, code_language)) => {
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

                    (cached_code, code_language, generated_clex)
                }
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

                    let code_llm_generator =
                        clex_llm::create_code_generator(gemini_api_key.expose_secret())
                            .map_err(|e| EvaluateAPIError::ClexLLMError(e.to_string()))?;

                    let (generated_code, generated_clex) = tokio::join!(
                        clex_llm::generate_code_solution(
                            &code_llm_generator,
                            &scrape_clex.statement,
                            &scrape_clex.input_format,
                            &scrape_clex.constraints,
                        ),
                        clex_llm::generate_clex_expression(
                            &clex_llm_generator,
                            &scrape_clex.input_format,
                            &scrape_clex.constraints,
                        )
                    );

                    let (generated_code, generated_language) = generated_code
                        .map_err(|e| EvaluateAPIError::ClexLLMError(e.to_string()))?;

                    let generated_clex = generated_clex
                        .map_err(|e| EvaluateAPIError::ClexLLMError(e.to_string()))?;

                    verify_clex(&generated_clex)?;

                    // Run both cache operations concurrently
                    let (clex_result, scrape_result, code_gen_result) = tokio::join!(
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
                        ),
                        cache_code_gen_llm_into_db(
                            &pool,
                            &code_request.problem_url,
                            &generated_code,
                            generated_language.clone()
                        )
                    );

                    // Check results of both operations
                    clex_result?;
                    scrape_result?;
                    code_gen_result.map_err(|e| EvaluateAPIError::ClexLLMError(e.to_string()))?;

                    (generated_code, generated_language, generated_clex)
                }
            }
        }
    };

    let response = run_and_compare(
        &correct_code_llm,
        &code_request.test_code,
        lang_name,
        code_request.test_code_language.clone(),
        &clex,
    )?;

    Ok(HttpResponse::Ok().json(response))
}

#[tracing::instrument(name = "Try getting cached scrape from DB", skip(pool))]
async fn get_cached_all_data_scrape_from_db(
    pool: &PgPool,
    question_url: &str,
) -> Result<Option<ScrapeCacheStoreResponse>, anyhow::Error> {
    let query = sqlx::query!(
        r#"
        SELECT statement, input_format, constraints, clex
        FROM scrape_cache
        WHERE question_url = $1 
        AND scraped_at + ttl > NOW()
        LIMIT 1;
        "#,
        question_url
    );

    let result = query
        .fetch_optional(pool)
        .await
        .context("Failed to fetch scrape details")?;

    Ok(result.map(|row| ScrapeCacheStoreResponse {
        statement: row.statement,
        input_format: row.input_format,
        constraints: row.constraints,
        clex: row.clex,
    }))
}

#[tracing::instrument(name = "Try getting cached LLM generated code from DB", skip(pool))]
pub(crate) async fn get_cached_code_gen_llm(
    pool: &PgPool,
    question_url: &str,
) -> Result<Option<(String, LanguageName)>, anyhow::Error> {
    let query = sqlx::query!(
        r#"
        SELECT code, language
        FROM code_gen_llm_cache
        WHERE question_url = $1 
        AND generated_at + ttl > NOW()
        LIMIT 1;
        "#,
        question_url
    );

    let result = query
        .fetch_optional(pool)
        .await
        .context("Failed to fetch scrape details")?;

    let result = match result {
        Some(row) => {
            let language = LanguageName::try_from(row.language)
                .map_err(|_| EvaluateAPIError::InvalidLanguageNameInDB)?;
            Some((row.code, language))
        }
        None => None,
    };

    Ok(result)
}

#[tracing::instrument(name = "Cache code generated by LLM into DB", skip(pool))]
pub(crate) async fn cache_code_gen_llm_into_db(
    pool: &PgPool,
    question_url: &str,
    code: &str,
    language: LanguageName,
) -> Result<(), anyhow::Error> {
    let mut transaction: Transaction<'_, Postgres> =
        pool.begin().await.context("Failed to start transaction")?;

    let query = sqlx::query!(
        r#"
        INSERT INTO code_gen_llm_cache (question_url, code, language)
        VALUES ($1, $2, $3)
        ON CONFLICT (question_url) DO UPDATE 
        SET code = $2, language = $3, generated_at = NOW();
        "#,
        question_url,
        code,
        language.to_string()
    );

    transaction.execute(query).await?;
    transaction.commit().await?;

    Ok(())
}
