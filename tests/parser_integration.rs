use cpast::clex_language::ast::{DataType, RepetitionType, UnitExpression};
use cpast::get_ast;

#[test]
fn test_get_ast_with_complex_pattern() {
    let language = "(N[5,7]) (?: (N[1,5]) N[-10000,10000]{\\2}){\\1}";
    let ast = get_ast(language.to_string());

    assert_eq!(
        ast.expression,
        vec![
            UnitExpression::CapturingGroup {
                group_number: 1,
                data_type: DataType::Integer(5, 7),
            },
            UnitExpression::NonCapturingGroup {
                nest_exp: vec![
                    UnitExpression::CapturingGroup {
                        group_number: 2,
                        data_type: DataType::Integer(1, 5),
                    },
                    UnitExpression::Primitives {
                        data_type: DataType::Integer(-10000, 10000),
                        repetition: RepetitionType::ByGroup { group_number: 2 },
                    },
                ],
                repetition: RepetitionType::ByGroup { group_number: 1 },
            },
            UnitExpression::Eof,
        ]
    );
}

#[test]
fn test_get_ast_with_backreference() {
    let language = "S {\\1}";
    let ast = get_ast(language.to_string());

    assert_eq!(
        ast.expression,
        vec![
            UnitExpression::Primitives {
                data_type: DataType::String,
                repetition: RepetitionType::ByGroup { group_number: 1 },
            },
            UnitExpression::Eof
        ]
    );
}
