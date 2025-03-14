#![allow(dead_code, unused_variables)]
use actix_web::{ResponseError, http::StatusCode};
use ccode_runner::lang_runner::{
    language_name::LanguageName, program_store::ProgramStore, runner_error_types::RunnerErrorType,
};
use clex_gen::clex_language::{self, code_generator::Generator, lexer};
use serde::Serialize;
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
}

#[derive(thiserror::Error)]
pub enum EvaluateAPIError {
    #[error(transparent)]
    APIClexErrorType(#[from] clex_language::clex_error_type::ClexErrorType),

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
        }
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
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
        .map_err(EvaluateAPIError::APIClexErrorType)?;
    let mut parser = clex_language::parser::Parser::new_from_tokens(token);
    parser
        .parser()
        .map_err(EvaluateAPIError::APIClexErrorType)?;
    let generator = Generator::new(&parser);

    let mut response = EvaluateCodeResponse {
        has_output_matched: true,
        input_diffs: Vec::new(),
    };

    for _ in 0..10 {
        let testcase = generator
            .generate_testcases()
            .map_err(EvaluateAPIError::APIClexErrorType)?;
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
