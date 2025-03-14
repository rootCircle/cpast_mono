use crate::helpers::spawn_app;
use reqwest::StatusCode;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
struct EvaluateCodeInputDiff {
    input: String,
    expected_output: String,
    actual_output: String,
}

#[derive(Deserialize)]
struct EvaluateCodeResponse {
    has_output_matched: bool,
    input_diffs: Vec<EvaluateCodeInputDiff>,
}

#[derive(Deserialize)]
struct PostShareAPIResponse {
    share_id: String,
}

#[tokio::test]
async fn evaluate_with_shared_id_works() {
    let app = spawn_app().await;

    // First, create a shared code
    let share_body = serde_json::json!({
        "code": "print('Hello')",
        "language": "Python",
        "clex": "N[1,10]"
    });

    let share_response = app.post_shared_code(&share_body).await;
    assert_eq!(StatusCode::OK, share_response.status());
    let share_id: PostShareAPIResponse = share_response.json().await.unwrap();

    // Then test evaluation with the shared code
    let eval_body = serde_json::json!({
        "test_code": "print('Hello')",
        "test_code_language": "Python",
        "share_id": share_id.share_id
    });

    let response = app.post_evaluate_with_shared_id(&eval_body).await;
    assert_eq!(StatusCode::OK, response.status());

    let evaluation = response.json::<EvaluateCodeResponse>().await.unwrap();
    assert!(evaluation.has_output_matched);
    assert_eq!(evaluation.input_diffs.len(), 0);
}

#[tokio::test]
async fn evaluate_with_invalid_share_id_returns_400() {
    let app = spawn_app().await;

    let req_body = serde_json::json!({
        "test_code": "print('Hello')",
        "test_code_language": "Python",
        "share_id": "invalid-share-id"
    });

    let response = app.post_evaluate_with_shared_id(&req_body).await;
    assert_eq!(StatusCode::BAD_REQUEST, response.status());
}

#[tokio::test]
async fn evaluate_with_different_outputs() {
    let app = spawn_app().await;

    // First, create a shared code
    let share_body = serde_json::json!({
        "code": "print('Hello')",
        "language": "Python",
        "clex": "N[1,10]"
    });

    let share_response = app.post_shared_code(&share_body).await;
    assert_eq!(StatusCode::OK, share_response.status());
    let share_id: PostShareAPIResponse = share_response.json().await.unwrap();

    // Then test evaluation with different output
    let eval_body = serde_json::json!({
        "test_code": "print('World')",
        "test_code_language": "Python",
        "share_id": share_id.share_id
    });

    let response = app.post_evaluate_with_shared_id(&eval_body).await;
    assert_eq!(StatusCode::OK, response.status());

    let evaluation = response.json::<EvaluateCodeResponse>().await.unwrap();
    assert!(!evaluation.has_output_matched);
    assert!(!evaluation.input_diffs.is_empty());
    assert_eq!(evaluation.input_diffs[0].expected_output, "Hello\n");
    assert_eq!(evaluation.input_diffs[0].actual_output, "World\n");
}

#[tokio::test]
async fn evaluate_with_invalid_test_code_syntax() {
    let app = spawn_app().await;

    // First, create a shared code
    let share_body = serde_json::json!({
        "code": "print('Hello')",
        "language": "Python",
        "clex": "N[1,10]"
    });

    let share_response = app.post_shared_code(&share_body).await;
    assert_eq!(StatusCode::OK, share_response.status());
    let share_id: PostShareAPIResponse = share_response.json().await.unwrap();

    // Then test evaluation with invalid syntax
    let eval_body = serde_json::json!({
        "test_code": "print('Hello'",
        "test_code_language": "Python",
        "share_id": share_id.share_id
    });

    let response = app.post_evaluate_with_shared_id(&eval_body).await;
    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, response.status());
}
