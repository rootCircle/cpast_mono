use super::lexer::Tokens;

struct Language {
    tokens: Vec<Expression>
}

struct Expression {
    unit: Vec<UnitExpression>
}

enum UnitExpression {
    RangeBoundPrimitives {
        data_type: NumeralDataType,
        lower_bound: i64,
        upper_bound: i64
    },
    Primitives {
        data_type: DataType
    },
    NonCapturingGroup {
        nest_exp: Expression
    },
    CapturingGroup {
        // Type is fixed to be non-negative Number
        group_number: u64
    },
    RepeatingExpressions {
        unit_type: UnitExpression,
        frequency_value_type: BackReferenceType
    }
}

enum DataType {
    Integer,
    Float,
    String,
    Char
}

enum NumeralDataType {
    Integer,
    Float
}

enum BackReferenceType {
    Group {
        group_number: u64
    },
    Literal(u64)
}

impl Language {
    fn new() -> Self {
        Language {
            tokens: vec![]
        }
    }

    pub fn parser(&self, tokens: Tokens) -> Language{

        Language { tokens: vec![] }
    }

}