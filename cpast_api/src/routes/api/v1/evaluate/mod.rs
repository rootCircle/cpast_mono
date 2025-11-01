use actix_web::{ResponseError, http::StatusCode};
use anyhow::Context;
use ccode_runner::lang_runner::{
    language_name::LanguageName, program_store::ProgramStore, runner_error_types::RunnerErrorType,
};
use clex_gen::clex_language::{self, code_generator::Generator, lexer};
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::{Executor, PgPool, Postgres, Transaction};
use utoipa::{OpenApi, ToSchema};

pub(crate) mod with_code_and_clex;
pub(crate) mod with_code_and_constraint;
pub(crate) mod with_code_and_platform;
pub(crate) mod with_platform;
pub(crate) mod with_shared_id;

#[derive(OpenApi)]
#[openapi(paths(
    crate::routes::api::v1::evaluate::with_code_and_constraint::post_with_code_and_constraint,
    crate::routes::api::v1::evaluate::with_code_and_clex::post_with_code_and_clex,
    crate::routes::api::v1::evaluate::with_code_and_platform::post_with_code_and_platform,
    crate::routes::api::v1::evaluate::with_shared_id::post_with_shared_id,
    crate::routes::api::v1::evaluate::with_platform::post_with_platform,
))]
pub(crate) struct EvaluateCodeApiv1;

#[derive(Serialize, ToSchema)]
struct EvaluateCodeInputDiff {
    #[schema(example = "world")]
    input: String,

    #[schema(example = "Hello, world!")]
    expected_output: String,

    #[schema(example = "Hello, worldd!")]
    actual_output: String,
}

#[derive(Serialize, ToSchema)]
struct EvaluateCodeResponse {
    #[schema(example = false)]
    has_output_matched: bool,

    #[schema(example = json!(Vec::from([EvaluateCodeInputDiff {
        input: "world".to_string(),
        expected_output: "Hello, world!".to_string(),
        actual_output: "Hello, worldd!".to_string(),
    }])))]
    input_diffs: Vec<EvaluateCodeInputDiff>,

    #[schema(example = "N[1,1000]")]
    clex: String,
}

#[derive(thiserror::Error)]
pub enum EvaluateAPIError {
    #[error(transparent)]
    APIClexErrorType(#[from] clex_gen::ClexError),

    #[error(transparent)]
    ScrapperError(#[from] cscrapper::qscrapper::ScraperError),

    #[error("{0}")]
    DirtyLanguageInDatabase(String),

    #[error("{0}")]
    ShareIdNotFound(String),

    #[error("{0}")]
    InvalidShareId(String),

    #[error(transparent)]
    APIRunnerErrorType(#[from] Box<RunnerErrorType>),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),

    #[error("{0}")]
    ClexLLMError(String),

    #[error("Invalid input format or constraints provided")]
    InvalidInputFormatOrConstraints,

    #[error("Invalid problem URL")]
    InvalidProblemURL,
}

impl std::fmt::Debug for EvaluateAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for EvaluateAPIError {
    fn status_code(&self) -> StatusCode {
        match self {
            EvaluateAPIError::APIClexErrorType(_) => StatusCode::BAD_REQUEST,
            EvaluateAPIError::InvalidShareId(_) => StatusCode::BAD_REQUEST,
            EvaluateAPIError::DirtyLanguageInDatabase(_) => StatusCode::INTERNAL_SERVER_ERROR,
            EvaluateAPIError::ShareIdNotFound(_) => StatusCode::NOT_FOUND,
            EvaluateAPIError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            EvaluateAPIError::APIRunnerErrorType(_) => StatusCode::INTERNAL_SERVER_ERROR,
            EvaluateAPIError::ClexLLMError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            EvaluateAPIError::InvalidInputFormatOrConstraints => StatusCode::BAD_REQUEST,
            EvaluateAPIError::InvalidProblemURL => StatusCode::BAD_REQUEST,
            EvaluateAPIError::ScrapperError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{e}\n")?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{cause}")?;
        current = cause.source();
    }
    Ok(())
}

fn verify_clex(clex: &str) -> Result<(), EvaluateAPIError> {
    clex_gen::generator(clex.to_string()).map_err(EvaluateAPIError::APIClexErrorType)?;
    Ok(())
}

fn run_and_compare(
    correct_code: &str,
    test_code: &str,
    correct_code_language: LanguageName,
    test_code_language: LanguageName,
    clex_language: &str,
) -> Result<EvaluateCodeResponse, EvaluateAPIError> {
    let runner = ProgramStore::new_from_text(
        correct_code,
        test_code,
        correct_code_language.clone(),
        test_code_language.clone(),
        false,
    )
    .map_err(EvaluateAPIError::APIRunnerErrorType)?;

    let mut token = lexer::Tokens::new(clex_language.to_string());
    token
        .scan_tokens()
        .map_err(|e| EvaluateAPIError::APIClexErrorType(e.into()))?;
    let mut parser = clex_language::parser::Parser::new_from_tokens(token);
    parser
        .parser()
        .map_err(|e| EvaluateAPIError::APIClexErrorType(e.into()))?;
    let generator = Generator::new(&parser);

    let mut response = EvaluateCodeResponse {
        has_output_matched: true,
        input_diffs: Vec::new(),
        clex: clex_language.to_string(),
    };

    for _ in 0..10 {
        let testcase = generator
            .generate_testcases()
            .map_err(|e| EvaluateAPIError::APIClexErrorType(e.into()))?;
        let (matched, expected, actual) = runner
            .run_codes_and_compare_output(&testcase)
            .map_err(EvaluateAPIError::APIRunnerErrorType)?;
        if !matched {
            response.has_output_matched = false;
            response.input_diffs.push(EvaluateCodeInputDiff {
                input: testcase,
                expected_output: expected,
                actual_output: actual,
            });
        }
    }

    Ok(response)
}

fn hash_input_format_and_constraints(input_format: &str, constraints: &str) -> String {
    // To avoid duplicates we trim and lowercase all of them
    // This may be a problem for future me, but I am yet not that future me!
    // Can cause collisions, but profits are good!
    let mut input_constraint = input_format.trim().to_ascii_lowercase();
    input_constraint.push_str(&constraints.trim().to_ascii_lowercase());

    let mut hasher = Sha256::new();

    hasher.update(input_constraint);

    let result = hasher.finalize();

    hex::encode(result)
}

#[tracing::instrument(name = "Try getting cached clex scrapped from DB", skip(pool))]
pub(crate) async fn get_cached_clex_scraped_from_db(
    pool: &PgPool,
    question_url: &str,
) -> Result<Option<String>, anyhow::Error> {
    let query = sqlx::query!(
        r#"
        SELECT clex
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

    Ok(result.map(|row| row.clex))
}

#[tracing::instrument(name = "Cache scraped data into DB", skip(pool))]
pub(crate) async fn cache_scrape_into_db(
    pool: &PgPool,
    question_url: &str,
    input_format: &str,
    constraints: &str,
    statement: &str,
    clex: &str,
) -> Result<(), anyhow::Error> {
    let mut transaction: Transaction<'_, Postgres> =
        pool.begin().await.context("Failed to start transaction")?;

    let query = sqlx::query!(
        r#"
        INSERT INTO scrape_cache (question_url, input_format, constraints, statement, clex)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (question_url) DO UPDATE 
        SET input_format = $2, constraints = $3, statement = $4, scraped_at = NOW(), clex = $5;
        "#,
        question_url,
        input_format,
        constraints,
        statement,
        clex
    );

    transaction.execute(query).await?;
    transaction.commit().await?;

    Ok(())
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

#[tracing::instrument(name = "Cache llm generated clex into DB", skip(pool))]
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
