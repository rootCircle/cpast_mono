use serde::Deserialize;
use uuid::{Uuid, Version};

use crate::helpers::spawn_app;

#[derive(Deserialize)]
struct APIResponse {
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
        .json::<APIResponse>()
        .await
        .unwrap();

    assert_eq!(
        Some(Version::SortRand),
        Uuid::parse_str(response.share_id.as_str())
            .expect("Failed to parse UUID")
            .get_version()
    );
}
