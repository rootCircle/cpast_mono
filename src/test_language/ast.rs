pub(crate) struct AstLanguage {
    pub(crate) tokens: Vec<Expression>
}

#[derive(Debug)]
pub(crate) struct Expression {
    pub(crate) unit: Vec<UnitExpression>
}

#[derive(Debug)]
pub(crate) enum UnitExpression {
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
        unit_type: Box<UnitExpression>,
        frequency_value_type: BackReferenceType
    }
}

#[derive(Debug)]
pub(crate) enum DataType {
    Integer,
    Float,
    String,
    Character
}

#[derive(Debug)]
pub(crate) enum NumeralDataType {
    Integer,
    Float
}

#[derive(Debug)]
pub(crate) enum BackReferenceType {
    Group {
        group_number: u64
    },
    Literal(i64)
}
