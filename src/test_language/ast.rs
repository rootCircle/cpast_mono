pub(crate) struct Program {
    pub(crate) tokens: Vec<UnitExpression>
}

#[derive(Debug)]
pub(crate) enum UnitExpression {
    Primitives {
        data_type: DataType,
        repetition: RepetitionType
    },
    CapturingGroup {
        // Type is fixed to be non-negative Number
        group_number: u64,
        data_type: DataType // Need to ensure DataType must be DataType::Integer(i64>0, i64>0)
    },
    NonCapturingGroup {
        nest_exp: Vec<ChildUnitExpression>,
        repetition: RepetitionType
    }
}

#[derive(Debug)]
pub(crate) enum ChildUnitExpression {
    Primitives {
        data_type: DataType,
        repetition: RepetitionType
    },
    NonCapturingGroup {
        nest_exp: Vec<ChildUnitExpression>,
        repetition: RepetitionType
    }
}


#[derive(Debug)]
pub(crate) enum DataType {
    Integer(i64, i64), // Minimum value, Maximum Value (Inclusive)
    Float(f64, f64), // Minimum value, Maximum Value (Inclusive)
    String,
    Character
}

#[derive(Debug)]
pub(crate) enum RepetitionType {
    ByGroup {
        group_number: u64
    },
    ByCount(u64), // The number of times it's going to be repeated
    None // No Repetition, similar to Literal(1)
}
