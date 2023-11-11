//! The `ast` module defines the Abstract Syntax Tree (AST) for the custom language generator, known as `clex`.
//! This module represents the structured hierarchy of code patterns specified in the `clex` language.
//!
//! # Types
//!
//! - `Program`: The top-level AST type representing a program, consisting of a vector of `UnitExpression`.
//! - `UnitExpression`: Enumerates different types of expressions within a program, including primitives, capturing groups, non-capturing groups, and an end-of-file marker.
//! - `DataType`: Enumerates different data types that can be associated with expressions, such as integer, float, string, and character.
//! - `RepetitionType`: Enumerates different repetition types, including repetition by capturing group, repetition by count, and no repetition.
//!
//! The `ast` module provides a structured representation of the code patterns specified in the `clex` language,
//! making it easier for other components of the `clex_language` module, such as the parser and generator, to process and manipulate the input patterns.
//!
//! # Example
//!
//! ```rust
//! use cpast::clex_language::ast::{Program, UnitExpression, DataType, RepetitionType};
//!
//! // Define a simple program AST
//! let program_ast = Program {
//!     expression: vec![
//!         UnitExpression::Primitives {
//!             data_type: DataType::Integer(0, 100),
//!             repetition: RepetitionType::None,
//!         },
//!         UnitExpression::CapturingGroup {
//!             group_number: 1,
//!             data_type: DataType::Float(0.0, 1.0),
//!         },
//!         UnitExpression::Eof,
//!     ],
//! };
//! ```
//!
//! For more details on the AST types and their usage, refer to the documentation for each type.


/// Represents a program consisting of a vector of `UnitExpression`.
#[derive(Debug)]
pub struct Program {
    pub expression: Vec<UnitExpression>,
}

/// Represents various unit expressions within a program.
#[derive(Debug, Clone, PartialEq)]
pub enum UnitExpression {
    /// Primitive unit expression with specified data type and repetition type.
    Primitives {
        data_type: DataType,
        repetition: RepetitionType,
    },
    /// Capturing group unit expression with a group number and data type.
    CapturingGroup {
        // Type is fixed to be non-negative Number, DataType is implied to be integer
        // i.e. DataType::Integer(0, TOTAL_GROUP_COUNT)
        group_number: u64,
        data_type: DataType,
    },
    /// Non-capturing group unit expression with nested expressions and repetition type.
    NonCapturingGroup {
        nest_exp: Vec<UnitExpression>,
        repetition: RepetitionType,
    },
    /// Represents the end of the file in the program.
    Eof,
}

/// Represents the data type of a unit expression.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    /// Integer data type with a specified minimum and maximum value (inclusive).
    Integer(i64, i64),
    /// Float data type with a specified minimum and maximum value (inclusive).
    Float(f64, f64),
    /// String data type.
    String,
    /// Character data type.
    Character,
}

/// Represents the repetition type of a unit expression.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RepetitionType {
    /// Repetition based on a capturing group with a specified group number.
    ByGroup { group_number: u64 },
    /// Repetition based on a specified count.
    ByCount(u64),
    /// No repetition, similar to a literal count of 1.
    None,
}
