use cpast::generator;

#[test]
fn test_generator_with_integer_expression() {
    let language = "(N[3,3]) (?:N[3,3]){\\1}";

    // Validate the output_text based on the generated AST
    assert_eq!(generator(language.to_string()), "3 3 3 3");
}

#[test]
fn test_generator_with_float_expression() {
    let language = "F[1, 1]";

    // Validate the output_text based on the generated AST
    assert_eq!(generator(language.to_string()), "1");
}

#[test]
fn test_generator_with_string_expression() {
    let language = "S";

    // Validate the output_text based on the generated AST
    assert!(!generator(language.to_string()).is_empty());
}
