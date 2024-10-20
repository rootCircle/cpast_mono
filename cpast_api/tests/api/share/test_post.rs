use reqwest::StatusCode;
use serde::Deserialize;
use uuid::{Uuid, Version};

use crate::helpers::spawn_app;

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

    assert_eq!(
        Some(Version::SortRand),
        Uuid::parse_str(response.share_id.as_str())
            .expect("Failed to parse UUID")
            .get_version()
    );
}

#[tokio::test]
async fn share_code_invalid_clex() {
    let app = spawn_app().await;

    let req_body = serde_json::json!({
        "code": "fn main() { println!(\"Hello, world!\"); }",
        "language": "Rust",
        "clex": "N[10,5]"
    });
    let response_code = app.post_shared_code(&req_body).await.status();
    assert_eq!(StatusCode::BAD_REQUEST, response_code);
}

#[tokio::test]
async fn share_code_invalid_language() {
    let app = spawn_app().await;

    let req_body = serde_json::json!({
        "code": "fn main() { println!(\"Hello, world!\"); }",
        "language": "Rustacean",
        "clex": "N[10,20]"
    });
    let response_code = app.post_shared_code(&req_body).await.status();
    assert_eq!(StatusCode::BAD_REQUEST, response_code);
}
