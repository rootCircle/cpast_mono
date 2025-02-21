use crate::helpers::spawn_app;
use ccode_runner::lang_runner::language_name::LanguageName;
use reqwest::StatusCode;
use serde::Deserialize;
use uuid::{ContextV7, Timestamp, Uuid};

#[derive(Deserialize)]
struct GetShareAPIResponse {
    code: String,
    language: LanguageName,
    clex: String,
}

#[derive(Deserialize)]
struct PostShareAPIResponse {
    share_id: String,
}

#[tokio::test]
async fn share_code_works() {
    let app = spawn_app().await;

    let req_body = serde_json::json!({
        "code": "fn main() { println!(\"Hello, world!\"); }",
        "language": "Rust",
        "clex": "N[10,20]"
    });

    let response = app
        .post_shared_code(&req_body)
        .await
        .json::<PostShareAPIResponse>()
        .await
        .unwrap();

    let shared_code_response = app
        .get_shared_code(response.share_id.as_str())
        .await
        .json::<GetShareAPIResponse>()
        .await
        .unwrap();

    assert_eq!(
        req_body["code"].as_str().unwrap(),
        shared_code_response.code
    );
    assert_eq!(
        req_body["language"]
            .as_str()
            .unwrap()
            .to_string()
            .try_into(),
        Ok(shared_code_response.language)
    );
    assert_eq!(
        req_body["clex"].as_str().unwrap(),
        shared_code_response.clex
    );
}

#[tokio::test]
async fn share_code_invalid_share_id() {
    let app = spawn_app().await;

    let response_code = app.get_shared_code("123e").await.status();
    assert_eq!(StatusCode::BAD_REQUEST, response_code);
}

#[tokio::test]
async fn test_non_existent_share_id() {
    let app = spawn_app().await;
    let context = ContextV7::new();
    let share_id = Uuid::new_v7(Timestamp::now(&context)).to_string();
    let response_code = app.get_shared_code(&share_id).await.status();
    assert_eq!(StatusCode::NOT_FOUND, response_code);
}
