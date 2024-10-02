use cscrapper::{get_problem_statement, CodePlatform};

#[test]
#[ignore]
fn test_codeforces_1992_b() {
    let result = get_problem_statement(CodePlatform::CodeForces("1331", "B"));
    eprintln!("{:?}", result);
    assert!(result.is_ok());
    let response = result.unwrap();

    assert!(response.statement.contains("April's Fool"));
    assert!(response
        .input_format
        .contains("The input contains a single integer"));
    assert!(response.constraints.is_empty());
}

#[test]
#[ignore]
fn test_codeforces_nonexistent_problem() {
    let result = get_problem_statement(CodePlatform::CodeForces("9999", "Z"));

    assert!(result.is_err());
}
