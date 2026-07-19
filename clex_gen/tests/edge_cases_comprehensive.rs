use clex_gen::clex_language::ast::{
    CharacterSet, DataType, FloatReferenceType, PositiveReferenceType, ReferenceType, Repetition,
    UnitExpression,
};
use clex_gen::clex_language::lexer::{Span, TokenType};
use clex_gen::{generator, get_ast, get_tokens};

// ============================================================================
// 1. LEXER EDGE CASES & UNICODE HANDLING
// ============================================================================

#[test]
fn test_lexer_empty_and_whitespace() {
    let tokens_empty = get_tokens("".to_string()).unwrap();
    assert_eq!(tokens_empty.len(), 1);
    assert_eq!(tokens_empty[0].token_type, TokenType::Eof);

    let tokens_ws = get_tokens(" \t\n\r ".to_string()).unwrap();
    assert_eq!(tokens_ws.len(), 1);
    assert_eq!(tokens_ws[0].token_type, TokenType::Eof);
}

#[test]
fn test_lexer_escape_sequences_in_single_quotes() {
    let input = r#"'a\n\t\r\\\'\"0\a\b\f\vb'"#;
    let tokens = get_tokens(input.to_string()).unwrap();
    assert_eq!(tokens.len(), 2);
    if let TokenType::LiteralString(ref s) = tokens[0].token_type {
        assert_eq!(s, "a\n\t\r\\\'\"0\x07\x08\x0C\x0Bb");
    } else {
        panic!("Expected LiteralString token");
    }
}

#[test]
fn test_lexer_escape_sequences_in_double_quotes() {
    let input = r#""x\n\t\r\\\'\"0\a\b\f\vy""#;
    let tokens = get_tokens(input.to_string()).unwrap();
    assert_eq!(tokens.len(), 2);
    if let TokenType::LiteralText(ref s) = tokens[0].token_type {
        assert_eq!(s, "x\n\t\r\\\'\"0\x07\x08\x0C\x0By");
    } else {
        panic!("Expected LiteralText token");
    }
}

#[test]
fn test_lexer_unknown_escape_sequence() {
    let input = r"'a\zb'";
    let tokens = get_tokens(input.to_string()).unwrap();
    if let TokenType::LiteralString(ref s) = tokens[0].token_type {
        assert_eq!(s, "a\\b");
    } else {
        panic!("Expected LiteralString token");
    }
}

#[test]
fn test_lexer_charset_case_insensitivity() {
    let charsets = [
        ("@ch_alpha@", TokenType::CharacterSetAlpha),
        ("@CH_NUM@", TokenType::CharacterSetNumeric),
        ("@ch_newline@", TokenType::CharacterSetNewline),
        ("@CH_ALNUM@", TokenType::CharacterSetAlnum),
        ("@ch_upper@", TokenType::CharacterSetUpper),
        ("@CH_LOWER@", TokenType::CharacterSetLower),
        ("@ch_all@", TokenType::CharacterSetAll),
    ];

    for (src, expected_type) in charsets {
        let tokens = get_tokens(src.to_string()).unwrap();
        assert_eq!(tokens[0].token_type, expected_type);
    }
}

#[test]
fn test_lexer_unicode_and_emojis() {
    let input = r#""Hello 🌍" 'こんにちは' @CH_NUM@"#;
    let tokens = get_tokens(input.to_string()).unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(
        tokens[0].token_type,
        TokenType::LiteralText("Hello 🌍".to_string())
    );
    assert_eq!(
        tokens[1].token_type,
        TokenType::LiteralString("こんにちは".to_string())
    );
    assert_eq!(tokens[2].token_type, TokenType::CharacterSetNumeric);
}

#[test]
fn test_lexer_numeric_parsing() {
    // Zeroes
    let tokens = get_tokens("0 -0 0.0 -0.0".to_string()).unwrap();
    assert_eq!(tokens[0].token_type, TokenType::LiteralNumber(0));
    assert_eq!(tokens[1].token_type, TokenType::LiteralNumber(0));
    assert_eq!(tokens[2].token_type, TokenType::LiteralFloat(0.0));
    assert_eq!(tokens[3].token_type, TokenType::LiteralFloat(0.0));

    // Integer overflow error
    let overflow_input = "N[999999999999999999999999999999999999999, 1]";
    let err = generator(overflow_input.to_string()).unwrap_err();
    assert!(err.to_string().contains("failed to parse number"));
}

#[test]
fn test_lexer_missing_digit_after_minus() {
    let err = generator("N[-]".to_string()).unwrap_err();
    assert!(err.to_string().contains("expected a number after negative sign"));
}

// ============================================================================
// 2. PARSER & AST EDGE CASES
// ============================================================================

#[test]
fn test_parser_omitted_range_bounds() {
    // N[1,] -> min=1, max=default
    let ast1 = get_ast("N[1,]".to_string()).unwrap();
    assert_eq!(
        ast1.expression[0],
        UnitExpression::Primitives {
            data_type: DataType::Integer(
                ReferenceType::ByLiteral(1),
                ReferenceType::ByLiteral(i32::MAX as i64)
            ),
            repetition: Repetition::Exact(PositiveReferenceType::ByLiteral(1)),
            span: Span { start: 0, end: 1 },
        }
    );

    // N[,100] -> min=default, max=100
    let ast2 = get_ast("N[,100]".to_string()).unwrap();
    assert_eq!(
        ast2.expression[0],
        UnitExpression::Primitives {
            data_type: DataType::Integer(
                ReferenceType::ByLiteral(i32::MIN as i64),
                ReferenceType::ByLiteral(100)
            ),
            repetition: Repetition::Exact(PositiveReferenceType::ByLiteral(1)),
            span: Span { start: 0, end: 1 },
        }
    );

    // N[,] -> min=default, max=default
    let ast3 = get_ast("N[,]".to_string()).unwrap();
    assert_eq!(
        ast3.expression[0],
        UnitExpression::Primitives {
            data_type: DataType::Integer(
                ReferenceType::ByLiteral(i32::MIN as i64),
                ReferenceType::ByLiteral(i32::MAX as i64)
            ),
            repetition: Repetition::Exact(PositiveReferenceType::ByLiteral(1)),
            span: Span { start: 0, end: 1 },
        }
    );
}

#[test]
fn test_parser_omitted_float_range_bounds() {
    let ast = get_ast("F[1.5,] F[,2.5] F[,]".to_string()).unwrap();
    assert_eq!(
        ast.expression[0],
        UnitExpression::Primitives {
            data_type: DataType::Float(
                FloatReferenceType::ByLiteral(1.5),
                FloatReferenceType::ByLiteral(i32::MAX as f64)
            ),
            repetition: Repetition::Exact(PositiveReferenceType::ByLiteral(1)),
            span: Span { start: 0, end: 1 },
        }
    );
    assert_eq!(
        ast.expression[1],
        UnitExpression::Primitives {
            data_type: DataType::Float(
                FloatReferenceType::ByLiteral(i32::MIN as f64),
                FloatReferenceType::ByLiteral(2.5)
            ),
            repetition: Repetition::Exact(PositiveReferenceType::ByLiteral(1)),
            span: Span { start: 8, end: 9 },
        }
    );
}

#[test]
fn test_parser_string_modifiers_variations() {
    // S -> default bounds & charset
    let ast_s = get_ast("S".to_string()).unwrap();
    assert_eq!(
        ast_s.expression[0],
        UnitExpression::Primitives {
            data_type: DataType::String(
                PositiveReferenceType::ByLiteral(1),
                PositiveReferenceType::ByLiteral(12),
                CharacterSet::AlphaNumeric
            ),
            repetition: Repetition::Exact(PositiveReferenceType::ByLiteral(1)),
            span: Span { start: 0, end: 1 },
        }
    );

    // S[5, 'custom'] -> exact length 5, custom charset
    let ast_custom = get_ast("S[5, 'abc']".to_string()).unwrap();
    assert_eq!(
        ast_custom.expression[0],
        UnitExpression::Primitives {
            data_type: DataType::String(
                PositiveReferenceType::ByLiteral(5),
                PositiveReferenceType::ByLiteral(5),
                CharacterSet::Custom("abc".to_string())
            ),
            repetition: Repetition::Exact(PositiveReferenceType::ByLiteral(1)),
            span: Span { start: 0, end: 1 },
        }
    );

    // S[, 10, @CH_NUM@] -> omitted min
    let ast_omitted_min = get_ast("S[, 10, @CH_NUM@]".to_string()).unwrap();
    assert_eq!(
        ast_omitted_min.expression[0],
        UnitExpression::Primitives {
            data_type: DataType::String(
                PositiveReferenceType::ByLiteral(1),
                PositiveReferenceType::ByLiteral(10),
                CharacterSet::Numeric
            ),
            repetition: Repetition::Exact(PositiveReferenceType::ByLiteral(1)),
            span: Span { start: 0, end: 1 },
        }
    );
}

#[test]
fn test_parser_backreference_validation() {
    // Group 0 reference error
    let err_zero = generator("N{\\0}".to_string()).unwrap_err();
    assert!(err_zero.to_string().contains("group number must be positive"));

    // Forward group reference error
    let err_forward = generator("N{\\1} (N)".to_string()).unwrap_err();
    assert!(err_forward.to_string().contains("reference to undefined group 1"));

    // Out-of-bounds group number error
    let err_oob = generator("(N) N{\\2}".to_string()).unwrap_err();
    assert!(err_oob.to_string().contains("reference to undefined group 2"));
}

#[test]
fn test_parser_non_capturing_groups_and_alternation() {
    // Alternation with empty branch
    let ast = get_ast("(?: N | )".to_string()).unwrap();
    if let UnitExpression::NonCapturingGroup { ref branches, .. } = ast.expression[0] {
        assert_eq!(branches.len(), 2);
        assert_eq!(branches[0].len(), 1);
        assert_eq!(branches[1].len(), 0);
    } else {
        panic!("Expected NonCapturingGroup");
    }

    // Nested non-capturing groups
    let nested_ast = get_ast("(?: (?: N[1,2] | S[1,2] ) | F[1.0, 2.0] )".to_string()).unwrap();
    assert_eq!(nested_ast.expression.len(), 2); // Group + EOF
}

#[test]
fn test_parser_syntax_errors() {
    // Missing comma in range
    let err1 = generator("N[1 5]".to_string()).unwrap_err();
    assert!(err1.to_string().contains("expected comma in range expression"));

    // Capturing group starting with non-N (e.g. S)
    let err2 = generator("(S)".to_string()).unwrap_err();
    assert!(err2.to_string().contains("expected valid content after opening parenthesis"));
}

// ============================================================================
// 3. GENERATOR EDGE CASES & RUNTIME BEHAVIORS
// ============================================================================

#[test]
fn test_generator_invalid_range_min_greater_than_max() {
    // Integer min > max
    let err1 = generator("N[10, 1]".to_string()).unwrap_err();
    assert!(err1.to_string().contains("invalid range: minimum (10) is greater than maximum (1)"));

    // Float min > max
    let err2 = generator("F[10.5, 1.2]".to_string()).unwrap_err();
    assert!(err2.to_string().contains("invalid range"));

    // String min > max
    let err3 = generator("S[10, 1]".to_string()).unwrap_err();
    assert!(err3.to_string().contains("invalid range"));

    // Repetition count min > max
    let err4 = generator("N{10, 1}".to_string()).unwrap_err();
    assert!(err4.to_string().contains("invalid range"));
}

#[test]
fn test_generator_runtime_group_not_found() {
    // Group 1 is defined inside an alternation branch that is never taken if we force branch resolution
    // Here, group 1 is in branch 0. If branch 1 is taken, group 1 won't exist when evaluating \1
    // We test that referencing an unvisited group throws an error at runtime
    let mut saw_error = false;
    for _ in 0..50 {
        if generator("(?: (N[5,5]) | \"hello\" ) N[\\1, \\1]".to_string()).is_err() {
            saw_error = true;
            break;
        }
    }
    assert!(saw_error, "Should fail when referencing unpopulated group");
}

#[test]
fn test_generator_zero_length_and_zero_repetition() {
    // String length 0
    let out1 = generator("(N[0,0]) S[\\1]".to_string()).unwrap();
    assert_eq!(out1, "0 "); // Group outputs '0 ' then trailing space popped => "0 "

    // Repetition 0
    let out2 = generator("(N[0,0]) N{\\1}".to_string()).unwrap();
    assert_eq!(out2, "0");
}

#[test]
fn test_generator_custom_unicode_charset() {
    // String from Greek charset
    let out_greek = generator("S[5, 'αβγ']".to_string()).unwrap();
    assert_eq!(out_greek.chars().count(), 5);
    assert!(out_greek.chars().all(|c| ['α', 'β', 'γ'].contains(&c)));

    // String from Emoji charset
    let out_emoji = generator("S[4, '😀😁😂']".to_string()).unwrap();
    assert_eq!(out_emoji.chars().count(), 4);
    assert!(out_emoji.chars().all(|c| ['😀', '😁', '😂'].contains(&c)));

    // String from empty custom charset
    let out_empty = generator("S[5, '']".to_string()).unwrap();
    assert_eq!(out_empty, "");
}

#[test]
fn test_generator_text_formatting_and_spaces() {
    // Multiple literals output verbatim without spaces
    let out_lit = generator(r#""a" "b" "c""#.to_string()).unwrap();
    assert_eq!(out_lit, "abc");

    // Mix of primitives and literals
    let out_mix = generator(r#"(N[5,5]) " items: " S[3, 3, 'x'] "\n""#.to_string()).unwrap();
    assert_eq!(out_mix, "5  items: xxx \n");
}
