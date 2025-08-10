use cscrapper::{CodePlatform, get_problem_statement};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_codeforces_valid() {
    let result = get_problem_statement(CodePlatform::CodeForces("1331", "B")).await;
    assert!(result.is_ok());
    let response = result.unwrap();

    assert!(response.statement.contains("April's Fool"));
    assert!(
        response
            .input_format
            .contains("The input contains a single integer")
    );
    assert!(response.constraints.is_empty());
}

#[tokio::test]
#[serial]
async fn test_codeforces_nonexistent_problem() {
    let result = get_problem_statement(CodePlatform::CodeForces("9999", "Z")).await;

    assert!(result.is_err());
}
