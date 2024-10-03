use clex_llm::{create_generator, generate_clex_expression};
use std::env;

#[tokio::test]
async fn test_generate_clex_expression() {
    let api_key = env::var("GEMINI_API_KEY").ok();

    if api_key.is_none() {
        eprintln!("Skipping test_generate_clex_expression: GEMINI_API_KEY not set");
        return;
    }

    let generator =
        create_generator(api_key.as_deref().unwrap()).expect("Failed to create generator");

    let input_format = "The first line contains an integer K, followed by K lines each containing a floating-point number P.";
    let constraints = "1 ≤ K ≤ 50\n0.0 ≤ P ≤ 1000.0";

    let result = generate_clex_expression(&generator, input_format, constraints).await;

    match result {
        Ok(expression) => {
            assert_eq!(
                expression, "(N[1,50]) (?:F[0,1000]){\\1}",
                "Generated expression should not be empty"
            );
        }
        Err(e) => {
            panic!("Failed to generate Clex expression: {:?}", e);
        }
    }
}
