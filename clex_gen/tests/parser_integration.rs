use clex_gen::clex_language::ast::{
    CharacterSet, DataType, FloatReferenceType, PositiveReferenceType, ReferenceType, Repetition,
    UnitExpression,
};
use clex_gen::clex_language::lexer::Span;
use clex_gen::{clex_language, get_ast};

#[test]
fn test_get_ast_with_complex_pattern() {
    let language = "(N[5,7]) (?: (N[1,5]) N[-10000,10000]{\\2}){\\1}";
    let ast = get_ast(language.to_string()).unwrap();

    assert_eq!(
        ast.expression,
        vec![
            UnitExpression::CapturingGroup {
                group_number: 1,
                range: (
                    PositiveReferenceType::ByLiteral(5),
                    PositiveReferenceType::ByLiteral(7)
                ),
                span: Span { start: 0, end: 1 },
            },
            UnitExpression::NonCapturingGroup {
                branches: vec![vec![
                    UnitExpression::CapturingGroup {
                        group_number: 2,
                        range: (
                            PositiveReferenceType::ByLiteral(1),
                            PositiveReferenceType::ByLiteral(5)
                        ),
                        span: Span { start: 13, end: 14 },
                    },
                    UnitExpression::Primitives {
                        data_type: DataType::Integer(
                            ReferenceType::ByLiteral(-10000),
                            ReferenceType::ByLiteral(10000)
                        ),
                        repetition: Repetition::Exact(PositiveReferenceType::ByGroup {
                            group_number: 2
                        }),
                        span: Span { start: 22, end: 23 },
                    },
                ]],
                repetition: Repetition::Exact(PositiveReferenceType::ByGroup { group_number: 1 }),
                span: Span { start: 9, end: 10 },
            },
            UnitExpression::Eof,
        ]
    );
}

#[test]
fn test_get_ast_with_backreference() {
    let language = "(N) S {\\1}";
    let ast = get_ast(language.to_string()).unwrap();

    assert_eq!(
        ast.expression,
        vec![
            UnitExpression::CapturingGroup {
                group_number: 1,
                range: (
                    PositiveReferenceType::ByLiteral(
                        clex_language::ast::DEFAULT_POSITIVE_RANGE_MIN_VALUE
                    ),
                    PositiveReferenceType::ByLiteral(
                        clex_language::ast::DEFAULT_POSITIVE_RANGE_MAX_VALUE
                    ),
                ),
                span: Span { start: 0, end: 1 },
            },
            UnitExpression::Primitives {
                data_type: DataType::String(
                    PositiveReferenceType::ByLiteral(
                        clex_language::ast::DEFAULT_MIN_STRING_SIZE as u64
                    ),
                    PositiveReferenceType::ByLiteral(
                        clex_language::ast::DEFAULT_MAX_STRING_SIZE as u64
                    ),
                    CharacterSet::get_default_charset()
                ),
                repetition: Repetition::Exact(PositiveReferenceType::ByGroup { group_number: 1 }),
                span: Span { start: 4, end: 5 },
            },
            UnitExpression::Eof
        ]
    );
}

#[test]
fn test_custom_charset_with_string() {
    let language = "S[1, 10, 'asghdgad']";
    let ast = get_ast(language.to_string()).unwrap();

    assert_eq!(
        ast.expression,
        vec![
            UnitExpression::Primitives {
                data_type: DataType::String(
                    PositiveReferenceType::ByLiteral(1),
                    PositiveReferenceType::ByLiteral(10),
                    CharacterSet::Custom("asghdgad".to_string())
                ),
                repetition: Repetition::Exact(PositiveReferenceType::ByLiteral(1)),
                span: Span { start: 0, end: 1 },
            },
            UnitExpression::Eof
        ]
    );
}

#[test]
fn test_charset_with_string() {
    let language = "S[10, 10, @CH_ALL@]";
    let ast = get_ast(language.to_string()).unwrap();

    assert_eq!(
        ast.expression,
        vec![
            UnitExpression::Primitives {
                data_type: DataType::String(
                    PositiveReferenceType::ByLiteral(10),
                    PositiveReferenceType::ByLiteral(10),
                    CharacterSet::All
                ),
                repetition: Repetition::Exact(PositiveReferenceType::ByLiteral(1)),
                span: Span { start: 0, end: 1 },
            },
            UnitExpression::Eof
        ]
    );
}

#[test]
fn test_alternation_ast() {
    let language = "(?: N | S )";
    let ast = get_ast(language.to_string()).unwrap();

    assert_eq!(
        ast.expression,
        vec![
            UnitExpression::NonCapturingGroup {
                branches: vec![
                    vec![UnitExpression::Primitives {
                        data_type: DataType::Integer(
                            ReferenceType::ByLiteral(clex_language::ast::DEFAULT_RANGE_MIN_VALUE),
                            ReferenceType::ByLiteral(clex_language::ast::DEFAULT_RANGE_MAX_VALUE),
                        ),
                        repetition: Repetition::Exact(PositiveReferenceType::ByLiteral(1)),
                        span: Span { start: 4, end: 5 },
                    }],
                    vec![UnitExpression::Primitives {
                        data_type: DataType::String(
                            PositiveReferenceType::ByLiteral(
                                clex_language::ast::DEFAULT_MIN_STRING_SIZE as u64
                            ),
                            PositiveReferenceType::ByLiteral(
                                clex_language::ast::DEFAULT_MAX_STRING_SIZE as u64
                            ),
                            CharacterSet::get_default_charset(),
                        ),
                        repetition: Repetition::Exact(PositiveReferenceType::ByLiteral(1)),
                        span: Span { start: 8, end: 9 },
                    }],
                ],
                repetition: Repetition::Exact(PositiveReferenceType::ByLiteral(1)),
                span: Span { start: 0, end: 1 },
            },
            UnitExpression::Eof,
        ]
    );
}

#[test]
fn test_literal_text_ast() {
    let language = "\"YES\\n\"";
    let ast = get_ast(language.to_string()).unwrap();

    assert_eq!(
        ast.expression,
        vec![
            UnitExpression::Literal {
                text: "YES\n".to_string(),
                span: Span { start: 0, end: 7 },
            },
            UnitExpression::Eof,
        ]
    );
}

#[test]
fn test_quantifier_range_ast() {
    let language = "N{2, 5}";
    let ast = get_ast(language.to_string()).unwrap();

    assert_eq!(
        ast.expression,
        vec![
            UnitExpression::Primitives {
                data_type: DataType::Integer(
                    ReferenceType::ByLiteral(clex_language::ast::DEFAULT_RANGE_MIN_VALUE),
                    ReferenceType::ByLiteral(clex_language::ast::DEFAULT_RANGE_MAX_VALUE),
                ),
                repetition: Repetition::Range(
                    PositiveReferenceType::ByLiteral(2),
                    PositiveReferenceType::ByLiteral(5)
                ),
                span: Span { start: 0, end: 1 },
            },
            UnitExpression::Eof,
        ]
    );
}

#[test]
fn test_flexible_string_modifier_ast() {
    let language = "S[5] S[5, @CH_UPPER@]";
    let ast = get_ast(language.to_string()).unwrap();

    assert_eq!(
        ast.expression,
        vec![
            UnitExpression::Primitives {
                data_type: DataType::String(
                    PositiveReferenceType::ByLiteral(5),
                    PositiveReferenceType::ByLiteral(5),
                    CharacterSet::get_default_charset(),
                ),
                repetition: Repetition::Exact(PositiveReferenceType::ByLiteral(1)),
                span: Span { start: 0, end: 1 },
            },
            UnitExpression::Primitives {
                data_type: DataType::String(
                    PositiveReferenceType::ByLiteral(5),
                    PositiveReferenceType::ByLiteral(5),
                    CharacterSet::Uppercase,
                ),
                repetition: Repetition::Exact(PositiveReferenceType::ByLiteral(1)),
                span: Span { start: 5, end: 6 },
            },
            UnitExpression::Eof,
        ]
    );
}

#[test]
fn test_float_bounds_ast() {
    let language = "F[1.5, 2.7]";
    let ast = get_ast(language.to_string()).unwrap();

    assert_eq!(
        ast.expression,
        vec![
            UnitExpression::Primitives {
                data_type: DataType::Float(
                    FloatReferenceType::ByLiteral(1.5),
                    FloatReferenceType::ByLiteral(2.7),
                ),
                repetition: Repetition::Exact(PositiveReferenceType::ByLiteral(1)),
                span: Span { start: 0, end: 1 },
            },
            UnitExpression::Eof,
        ]
    );
}
