use cscrapper::{get_problem_statement, CodePlatform};

#[test]
fn test_codechef_nonnegprod() {
    let result = get_problem_statement(CodePlatform::CodeChef("NONNEGPROD"));
    eprintln!("{:?}", result);
    assert!(result.is_ok());
    let response = result.unwrap();

    // Check if the response contains expected content
    assert!(response.statement.contains("Alice has an array of"));
    assert!(response.input_format.contains("The first line of input"));
    assert!(response.constraints.contains(
        "- $1 \\leq T \\leq 100$\n- $2 \\leq N \\leq 10000$\n- $-1000 \\leq A_i \\leq 1000$"
    ));
}

#[test]
fn test_codechef_nonexistent_problem() {
    let result = get_problem_statement(CodePlatform::CodeChef("NONEXISTENTPROBLEM"));

    assert!(result.is_err());
}
