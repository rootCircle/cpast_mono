pub(crate) struct AstLanguage {
    pub(crate) tokens: Vec<Expression>
}

pub(crate) struct Expression {
    pub(crate) unit: Vec<UnitExpression>
}

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

pub(crate) enum DataType {
    Integer,
    Float,
    String,
    Character
}

pub(crate) enum NumeralDataType {
    Integer,
    Float
}

pub(crate) enum BackReferenceType {
    Group {
        group_number: u64
    },
    Literal(u64)
}
