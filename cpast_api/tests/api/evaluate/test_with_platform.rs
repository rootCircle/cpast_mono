use crate::helpers::spawn_app;
use reqwest::StatusCode;
use serde::Deserialize;
use serial_test::serial;

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct EvaluateCodeInputDiff {
    input: String,
    expected_output: String,
    actual_output: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct EvaluateCodeResponse {
    has_output_matched: bool,
    input_diffs: Vec<EvaluateCodeInputDiff>,
    clex: String,
}

#[tokio::test]
#[serial]
async fn evaluate_with_platform_works() {
    let app = spawn_app().await;

    let req_body = serde_json::json!({
        "test_code": "X, Y = map(int, input().split())\nprint(X + Y * 10)",
        "test_code_language": "Python",
        "problem_url": "https://www.codechef.com/problems/WAPEN"
    });

    let response = app.post_evaluate_with_platform(&req_body).await;

    assert_eq!(StatusCode::OK, response.status());

    let evaluation = response.json::<EvaluateCodeResponse>().await.unwrap();
    assert!(!evaluation.clex.is_empty());
}

#[tokio::test]
#[serial]
async fn evaluate_with_empty_problem_url_returns_400() {
    let app = spawn_app().await;

    let req_body = serde_json::json!({
        "test_code": "print('Hello')",
        "test_code_language": "Python",
        "problem_url": ""
    });

    let response = app.post_evaluate_with_platform(&req_body).await;

    assert_eq!(StatusCode::BAD_REQUEST, response.status());
}

#[tokio::test]
#[serial]
async fn evaluate_with_invalid_problem_url_returns_400() {
    let app = spawn_app().await;

    let req_body = serde_json::json!({
        "test_code": "print('Hello')",
        "test_code_language": "Python",
        "problem_url": "https://invalid-url.com/problems/test"
    });

    let response = app.post_evaluate_with_platform(&req_body).await;

    assert_eq!(StatusCode::BAD_REQUEST, response.status());
}

#[tokio::test]
#[serial]
async fn evaluate_with_invalid_code_syntax() {
    let app = spawn_app().await;

    let req_body = serde_json::json!({
        "test_code": "print('Hello'",
        "test_code_language": "Python",
        "problem_url": "https://www.codechef.com/problems/WAPEN"
    });

    let response = app.post_evaluate_with_platform(&req_body).await;

    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, response.status());
}

#[tokio::test]
#[serial]
async fn evaluate_caching_works() {
    let app = spawn_app().await;

    let req_body = serde_json::json!({
        "test_code": "X, Y = map(int, input().split())\nprint(X + Y * 10)",
        "test_code_language": "Python",
        "problem_url": "https://www.codechef.com/problems/WAPEN"
    });

    // First request should cache the results
    let first_response = app.post_evaluate_with_platform(&req_body).await;
    assert_eq!(StatusCode::OK, first_response.status());

    // Second request should use cached results
    let second_response = app.post_evaluate_with_platform(&req_body).await;
    assert_eq!(StatusCode::OK, second_response.status());

    let first_evaluation = first_response.json::<EvaluateCodeResponse>().await.unwrap();
    let second_evaluation = second_response
        .json::<EvaluateCodeResponse>()
        .await
        .unwrap();

    // Both responses should have the same CLEX
    assert_eq!(first_evaluation.clex, second_evaluation.clex);
}
