#[derive(Debug)]
pub struct Program {
    pub(crate) expression: Vec<UnitExpression>,
}

#[derive(Debug, Clone)]
pub(crate) enum UnitExpression {
    Primitives {
        data_type: DataType,
        repetition: RepetitionType,
    },
    CapturingGroup {
        // Type is fixed to be non-negative Number, DataType is implied to be integer
        // i.e. DataType::Integer(0, TOTAL_GROUP_COUNT)
        group_number: u64,
        data_type: DataType,
    },
    NonCapturingGroup {
        nest_exp: Vec<UnitExpression>,
        repetition: RepetitionType,
    },
    Eof,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum DataType {
    Integer(i64, i64), // Minimum value, Maximum Value (Inclusive)
    Float(f64, f64),   // Minimum value, Maximum Value (Inclusive)
    String,
    Character,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum RepetitionType {
    ByGroup { group_number: u64 },
    ByCount(u64), // The number of times it's going to be repeated
    None,         // No Repetition, similar to Literal(1)
}
