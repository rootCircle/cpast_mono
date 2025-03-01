//! The `ast` module defines the Abstract Syntax Tree (AST) for the custom language generator, known as `clex`.
//! This module represents the structured hierarchy of code patterns specified in the `clex` language.
//!
//! # Types
//!
//! - `ClexLanguageAST`: The top-level AST type representing a program, consisting of a vector of `UnitExpression`.
//! - `UnitExpression`: Enumerates different types of expressions within a program, including primitives, capturing groups, non-capturing groups, and an end-of-file marker.
//! - `DataType`: Enumerates different data types that can be associated with expressions, such as integer, float, string, and character.
//! - `ReferenceType`: Enumerates different repetition types, including repetition by capturing group, repetition by count, and no repetition.
//! - `PositiveReferenceType`: Same as `ReferenceType`, but guarantees positive value upon de-referencing.
//!
//! The `ast` module provides a structured representation of the code patterns specified in the `clex` language,
//! making it easier for other components of the `clex_language` module, such as the parser and generator, to process and manipulate the input patterns.
//!
//! # Example
//!
//! ```rust
//! use clex_gen::clex_language::ast::{ClexLanguageAST, UnitExpression, DataType, ReferenceType, PositiveReferenceType};
//!
//! // Define a simple program AST
//! let program_ast = ClexLanguageAST {
//!     expression: vec![
//!         UnitExpression::Primitives {
//!             data_type: DataType::Integer(ReferenceType::ByLiteral(0), ReferenceType::ByLiteral(100)),
//!             repetition: PositiveReferenceType::ByLiteral(1),
//!         },
//!         UnitExpression::CapturingGroup {
//!             group_number: 1,
//!             range: (PositiveReferenceType::ByLiteral(0), PositiveReferenceType::ByLiteral(10)),
//!         },
//!         UnitExpression::Eof,
//!     ],
//! };
//! ```
//!
//! For more details on the AST types and their usage, refer to the documentation for each type.

// /// Represents a program consisting of a vector of `UnitExpression`.

//////////////////////////////////////////
// CONSTANTS
//////////////////////////////////////////
pub const DEFAULT_MIN_STRING_SIZE: usize = 1;
pub const DEFAULT_MAX_STRING_SIZE: usize = 12;
pub const DEFAULT_CHARSET: CharacterSet = CharacterSet::AlphaNumeric;
pub const DEFAULT_QUANTIFIER_VALUE: u64 = 1;
pub const DEFAULT_RANGE_MIN_VALUE: i64 = i32::MIN as i64;
pub const DEFAULT_RANGE_MAX_VALUE: i64 = i32::MAX as i64;
pub const DEFAULT_POSITIVE_RANGE_MIN_VALUE: u64 = u32::MIN as u64;
pub const DEFAULT_POSITIVE_RANGE_MAX_VALUE: u64 = u32::MAX as u64;

#[derive(Debug, Clone)]
pub struct ClexLanguageAST {
    pub expression: Vec<UnitExpression>,
}

/// Represents various unit expressions within a program.
#[derive(Debug, Clone, PartialEq)]
pub enum UnitExpression {
    /// Primitive unit expression with specified data type and repetition type.
    Primitives {
        data_type: DataType,
        repetition: PositiveReferenceType,
    },
    /// Capturing group unit expression with a group number and range.
    CapturingGroup {
        // Type is fixed to be non-negative Number, DataType is implied to be integer
        // DataType::Integer
        // group_number won't exceed the value of total number of capturing group present before.
        group_number: u64,
        range: (PositiveReferenceType, PositiveReferenceType),
    },
    /// Non-capturing group unit expression with nested expressions and repetition type.
    NonCapturingGroup {
        nest_exp: Vec<UnitExpression>,
        repetition: PositiveReferenceType,
    },
    /// Represents the end of the file in the program.
    Eof,
}

/// Represents the data type of unit expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataType {
    /// Integer data type with a specified minimum and maximum value (inclusive).
    Integer(ReferenceType, ReferenceType),
    /// Float data type with a specified minimum and maximum value (inclusive).
    Float(ReferenceType, ReferenceType),
    /// String data type with specified minimum and maximum length.
    String(PositiveReferenceType, PositiveReferenceType, CharacterSet),
}

/// Represents the repetition type of unit expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceType {
    /// Reference based on a capturing group with a specified group number.
    ByGroup { group_number: u64 },
    /// Reference based on a specified literal.
    ByLiteral(i64),
}

/// Represents the repetition type of unit expression, which is guaranteed to dereference to a positive value only!
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PositiveReferenceType {
    /// Reference based on a capturing group with a specified group number.
    ByGroup { group_number: u64 },
    /// Reference based on a specified literal.
    ByLiteral(u64),
}

/// Represent character set for string domain
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharacterSet {
    // CH_ALPHA
    Alphabet,
    // CH_NUM
    Numeric,
    // CH_NEWLINE
    Newline,
    // CH_ALNUM
    AlphaNumeric,
    // CH_UPPER
    Uppercase,
    // CH_LOWER
    LowerCase,
    // CH_ALL
    All,
    // "<characters here>"
    Custom(String),
}

impl CharacterSet {
    pub fn get_default_charset() -> CharacterSet {
        DEFAULT_CHARSET
    }

    pub fn get_character_domain(&self) -> String {
        match self {
            CharacterSet::Alphabet => {
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".to_string()
            }
            CharacterSet::Numeric => "0123456789".to_string(),
            CharacterSet::AlphaNumeric => {
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_string()
            }
            CharacterSet::Uppercase => "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
            CharacterSet::LowerCase => "abcdefghijklmnopqrstuvwxyz".to_string(),
            CharacterSet::All => {
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~"
                    .to_string()
            }
            CharacterSet::Newline => "\n".to_string(),
            CharacterSet::Custom(charset) => charset.to_owned(),
        }
    }
}
