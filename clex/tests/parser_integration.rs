use clex::clex_language::ast::{
    CharacterSet, DataType, PositiveReferenceType, ReferenceType, UnitExpression,
};
use clex::{clex_language, get_ast};

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
            },
            UnitExpression::NonCapturingGroup {
                nest_exp: vec![
                    UnitExpression::CapturingGroup {
                        group_number: 2,
                        range: (
                            PositiveReferenceType::ByLiteral(1),
                            PositiveReferenceType::ByLiteral(5)
                        ),
                    },
                    UnitExpression::Primitives {
                        data_type: DataType::Integer(
                            ReferenceType::ByLiteral(-10000),
                            ReferenceType::ByLiteral(10000)
                        ),
                        repetition: PositiveReferenceType::ByGroup { group_number: 2 },
                    },
                ],
                repetition: PositiveReferenceType::ByGroup { group_number: 1 },
            },
            UnitExpression::Eof,
        ]
    );
}

#[test]
fn test_get_ast_with_backreference() {
    let language = "S {\\1}";
    let ast = get_ast(language.to_string()).unwrap();

    assert_eq!(
        ast.expression,
        vec![
            UnitExpression::Primitives {
                data_type: DataType::String(
                    PositiveReferenceType::ByLiteral(clex_language::ast::MAX_STRING_SIZE as u64),
                    CharacterSet::get_default_charset()
                ),
                repetition: PositiveReferenceType::ByGroup { group_number: 1 },
            },
            UnitExpression::Eof
        ]
    );
}

#[test]
fn test_custom_charset_with_string() {
    let language = "S[10, 'asghdgad']";
    let ast = get_ast(language.to_string()).unwrap();

    assert_eq!(
        ast.expression,
        vec![
            UnitExpression::Primitives {
                data_type: DataType::String(
                    PositiveReferenceType::ByLiteral(10),
                    CharacterSet::Custom("asghdgad".to_string())
                ),
                repetition: PositiveReferenceType::ByLiteral(1),
            },
            UnitExpression::Eof
        ]
    );
}

#[test]
fn test_charset_with_string() {
    let language = "S[10, @CH_ALL@]";
    let ast = get_ast(language.to_string()).unwrap();

    assert_eq!(
        ast.expression,
        vec![
            UnitExpression::Primitives {
                data_type: DataType::String(
                    PositiveReferenceType::ByLiteral(10),
                    CharacterSet::All
                ),
                repetition: PositiveReferenceType::ByLiteral(1),
            },
            UnitExpression::Eof
        ]
    );
}
