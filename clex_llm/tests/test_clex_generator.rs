use clex_llm::{create_clex_generator, generate_clex_expression};
use std::env;

#[tokio::test]
async fn test_generate_clex_expression() {
    let api_key = env::var("GOOGLE_API_KEY").ok();

    if api_key.is_none() {
        eprintln!("Skipping test_generate_clex_expression: GOOGLE_API_KEY not set");
        return;
    }

    let generator =
        create_clex_generator(api_key.as_deref().unwrap()).expect("Failed to create generator");

    let input_format = "The first line contains an integer K, followed by K lines each containing a floating-point number P.";
    let constraints = "1 ≤ K ≤ 100\n0.0 ≤ P ≤ 1000.0";

    let result = generate_clex_expression(&generator, input_format, constraints).await;

    match result {
        Ok(expression) => {
            assert_eq!(
                expression, "(N[1,100]) (?:F[0,1000]){\\1}",
                "Generated expression should not be empty"
            );
        }
        Err(e) => {
            panic!("Failed to generate Clex expression: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_array_sum_generation() {
    let api_key = env::var("GOOGLE_API_KEY").ok();

    if api_key.is_none() {
        eprintln!("Skipping test_array_sum_generation: GOOGLE_API_KEY not set");
        return;
    }

    let generator =
        create_clex_generator(api_key.as_deref().unwrap()).expect("Failed to create generator");

    let input_format = "First line contains N denoting the size of array. Second line contains N space-separated integers.";
    let constraints = "1 ≤ N ≤ 1000\n-10^9 ≤ array elements ≤ 10^9";

    let result = generate_clex_expression(&generator, input_format, constraints).await;

    match result {
        Ok(expression) => {
            assert_eq!(
                expression, "(N[1,1000]) (?:N[-1000000000,1000000000]){\\1}",
                "Generated expression should match expected format"
            );
        }
        Err(e) => {
            panic!("Failed to generate Clex expression: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_string_pattern_generation() {
    let api_key = env::var("GOOGLE_API_KEY").ok();

    if api_key.is_none() {
        eprintln!("Skipping test_string_pattern_generation: GOOGLE_API_KEY not set");
        return;
    }

    let generator =
        create_clex_generator(api_key.as_deref().unwrap()).expect("Failed to create generator");

    let input_format = "First line contains T test cases. Each test case contains a string S consisting of only uppercase letters.";
    let constraints = "1 ≤ T ≤ 100\n|S| = 10";

    let result = generate_clex_expression(&generator, input_format, constraints).await;

    match result {
        Ok(expression) => {
            assert_eq!(
                expression, "(N[1,100]) (?:S[10,10,@CH_UPPER@]){\\1}",
                "Generated expression should match expected format"
            );
        }
        Err(e) => {
            panic!("Failed to generate Clex expression: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_matrix_generation() {
    let api_key = env::var("GOOGLE_API_KEY").ok();

    if api_key.is_none() {
        eprintln!("Skipping test_matrix_generation: GOOGLE_API_KEY not set");
        return;
    }

    let generator =
        create_clex_generator(api_key.as_deref().unwrap()).expect("Failed to create generator");

    let input_format = "First line contains N and M, dimensions of matrix. Next N lines contain M space-separated floating point numbers each.";
    let constraints = "1 ≤ N,M ≤ 100\n0.0 ≤ matrix elements ≤ 1.0";

    let result = generate_clex_expression(&generator, input_format, constraints).await;

    match result {
        Ok(expression) => {
            assert_eq!(
                expression, "(N[1,100]) (N[1,100]) (?:(?:F[0,1]){\\2}){\\1}",
                "Generated expression should match expected format"
            );
        }
        Err(e) => {
            panic!("Failed to generate Clex expression: {:?}", e);
        }
    }
}
