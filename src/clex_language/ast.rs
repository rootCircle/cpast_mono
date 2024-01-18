//! The `ast` module defines the Abstract Syntax Tree (AST) for the custom language generator, known as `clex`.
//! This module represents the structured hierarchy of code patterns specified in the `clex` language.
//!
//! # Types
//!
//! - `Program`: The top-level AST type representing a program, consisting of a vector of `UnitExpression`.
//! - `UnitExpression`: Enumerates different types of expressions within a program, including primitives, capturing groups, non-capturing groups, and an end-of-file marker.
//! - `DataType`: Enumerates different data types that can be associated with expressions, such as integer, float, string, and character.
//! - `ReferenceType`: Enumerates different repetition types, including repetition by capturing group, repetition by count, and no repetition.
//!
//! The `ast` module provides a structured representation of the code patterns specified in the `clex` language,
//! making it easier for other components of the `clex_language` module, such as the parser and generator, to process and manipulate the input patterns.
//!
//! # Example
//!
//! ```rust
//! use cpast::clex_language::ast::{Program, UnitExpression, DataType, ReferenceType};
//!
//! // Define a simple program AST
//! let program_ast = Program {
//!     expression: vec![
//!         UnitExpression::Primitives {
//!             data_type: DataType::Integer(ReferenceType::ByLiteral(0), ReferenceType::ByLiteral(100)),
//!             repetition: ReferenceType::ByLiteral(1),
//!         },
//!         UnitExpression::CapturingGroup {
//!             group_number: 1,
//!             data_type: DataType::Float(ReferenceType::ByLiteral(0), ReferenceType::ByLiteral(1)),
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
        repetition: ReferenceType,
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
        repetition: ReferenceType,
    },
    /// Represents the end of the file in the program.
    Eof,
}

/// Represents the data type of a unit expression.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    /// Integer data type with a specified minimum and maximum value (inclusive).
    Integer(ReferenceType, ReferenceType),
    /// Float data type with a specified minimum and maximum value (inclusive).
    Float(ReferenceType, ReferenceType),
    /// String data type.
    String,
    /// Character data type.
    Character,
}

/// Represents the repetition type of a unit expression.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReferenceType {
    /// Reference based on a capturing group with a specified group number.
    ByGroup { group_number: u64 },
    /// Reference based on a specified literal.
    ByLiteral(i64),
    /// No repetition, defaults to 1 in case of Repetitions
    None,
}
