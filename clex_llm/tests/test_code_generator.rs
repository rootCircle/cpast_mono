use ccode_runner::lang_runner::{language_name::LanguageName, program_store::ProgramStore};
use clex_llm::{create_code_generator, generate_code_solution};
use std::env;
#[tokio::test]
async fn test_generate_code_solution() {
    let api_key = env::var("GOOGLE_API_KEY").ok();

    if api_key.is_none() {
        eprintln!("Skipping test_generate_code_solution: GOOGLE_API_KEY not set");
        return;
    }

    let generator =
        create_code_generator(api_key.as_deref().unwrap()).expect("Failed to create generator");

    let statement = "Given an array of integers, find the sum of all elements.";
    let input_format = "First line contains N denoting the size of array. Second line contains N space-separated integers.";
    let constraints = "1 ≤ N ≤ 1000\n-10^9 ≤ array elements ≤ 10^9";

    let result = generate_code_solution(&generator, statement, input_format, constraints).await;

    match result {
        Ok((solution, language_name)) => {
            let correct_code = r#"n = int(input())
arr = list(map(int, input().split()))
print(sum(arr))"#;
            let runner = ProgramStore::new_from_text(
                correct_code,
                &solution,
                LanguageName::Python,
                language_name,
                false,
            )
            .unwrap();

            let (has_matched, _, _) = runner
                .run_codes_and_compare_output("10\n1 2 3 4 5 6 7 8 9 10")
                .unwrap();
            assert!(has_matched);
        }
        Err(e) => {
            panic!("Failed to generate code solution: {:?}", e);
        }
    }
}
