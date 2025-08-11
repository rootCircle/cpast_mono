use crate::helpers::{is_gemini_quota_error, spawn_app};
use flaky_test::flaky_test;
use reqwest::StatusCode;
use serde::Deserialize;
use serial_test::serial;

#[derive(Deserialize)]
struct EvaluateCodeInputDiff {
    #[allow(unused)]
    input: String,
    expected_output: String,
    actual_output: String,
}

#[derive(Deserialize)]
struct EvaluateCodeResponse {
    has_output_matched: bool,
    input_diffs: Vec<EvaluateCodeInputDiff>,
    clex: String,
}

#[flaky_test(times = 3, tokio)]
#[serial]
async fn evaluate_with_code_and_platform_works() {
    let app = spawn_app().await;

    let req_body = serde_json::json!({
        "correct_code": "print('Hello')",
        "correct_code_language": "Python",
        "test_code": "print('Hello')",
        "test_code_language": "Python",
        "problem_url": "https://www.codechef.com/problems/WAPEN"
    });

    let response = app.post_evaluate_with_code_and_platform(&req_body).await;
    if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
        let body = response.text().await.unwrap_or_default();
        if is_gemini_quota_error(&body) {
            eprintln!(
                "Skipping evaluate_with_code_and_platform_works due to Gemini rate limit: {body}"
            );
            return;
        }
        panic!("Unexpected 500: {body}");
    }
    assert_eq!(StatusCode::OK, response.status());

    let evaluation = response.json::<EvaluateCodeResponse>().await.unwrap();
    assert!(evaluation.has_output_matched);
    assert_eq!(evaluation.input_diffs.len(), 0);
    assert!(!evaluation.clex.is_empty());
}

#[flaky_test(times = 3, tokio)]
#[serial]
async fn evaluate_with_invalid_url_returns_400() {
    let app = spawn_app().await;

    let req_body = serde_json::json!({
        "correct_code": "print('Hello')",
        "correct_code_language": "Python",
        "test_code": "print('Hello')",
        "test_code_language": "Python",
        "problem_url": ""
    });

    let response = app.post_evaluate_with_code_and_platform(&req_body).await;

    assert_eq!(StatusCode::BAD_REQUEST, response.status());
}

#[flaky_test(times = 3, tokio)]
#[serial]
async fn evaluate_with_different_outputs() {
    let app = spawn_app().await;

    let req_body = serde_json::json!({
        "correct_code": "print('Hello')",
        "correct_code_language": "Python",
        "test_code": "print('World')",
        "test_code_language": "Python",
        "problem_url": "https://www.codechef.com/problems/WAPEN"
    });

    let response = app.post_evaluate_with_code_and_platform(&req_body).await;
    if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
        let body = response.text().await.unwrap_or_default();
        if is_gemini_quota_error(&body) {
            eprintln!("Skipping evaluate_with_different_outputs due to Gemini rate limit: {body}");
            return;
        }
        panic!("Unexpected 500: {body}");
    }
    assert_eq!(StatusCode::OK, response.status());

    let evaluation = response.json::<EvaluateCodeResponse>().await.unwrap();
    assert!(!evaluation.has_output_matched);
    assert!(!evaluation.input_diffs.is_empty());
    assert_eq!(evaluation.input_diffs[0].expected_output, "Hello\n");
    assert_eq!(evaluation.input_diffs[0].actual_output, "World\n");
    assert!(!evaluation.clex.is_empty());
}

#[flaky_test(times = 3, tokio)]
#[serial]
async fn evaluate_code_invalid_syntax() {
    let app = spawn_app().await;

    let req_body = serde_json::json!({
        "correct_code": "print('Hello'",
        "correct_code_language": "Python",
        "test_code": "print('Hello')",
        "test_code_language": "Python",
        "problem_url": "https://www.codechef.com/problems/WAPEN"
    });

    let response = app.post_evaluate_with_code_and_platform(&req_body).await;

    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, response.status());
}
