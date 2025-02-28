//! Error handling module for the clex language implementation.
//!
//! This module provides error types and implementations for handling various errors
//! that can occur during lexical analysis, parsing, and code generation phases.

use crate::clex_language::lexer::TokenType;
use core::fmt;
use std::error::Error;
/// Represents the main categories of errors that can occur in the CLEX system.
#[derive(Debug)]
pub enum ParentErrorType {
    /// Errors that occur during lexical analysis
    LexerError,
    /// Errors that occur during parsing
    ParserError,
    /// Errors that occur during code generation
    GeneratorError,
}

/// Specific error types that can occur during CLEX processing.
#[derive(Debug)]
pub enum ClexErrorType {
    // Lexical Errors
    /// Error when a single quote is not properly closed
    UnclosedSingleQuotes(ParentErrorType),
    /// Error when a colon is missing after a question mark
    MissingColonAfterQuestionMark(ParentErrorType),
    /// Error when a negative sign is not followed by a number
    MissingNumberAfterNegativeSign(ParentErrorType),
    /// Error when parsing numeric values
    NumericParsingError(ParentErrorType),
    /// Error when encountering an unknown character
    UnknownCharacter(ParentErrorType, &'static str),
    /// Error when an @ symbol is not properly closed
    UnclosedAtSymbol(ParentErrorType),
    /// Error when an invalid character set is specified, all valid character sets are specified in <https://github.com/rootCircle/cpast_mono/blob/main/clex_gen/docs/CLEX_LANG_SPECS.md#character>
    InvalidCharacterSet(ParentErrorType),

    // Parser Errors
    /// Error when a non-capturing group is missing closing parenthesis
    MissingClosingParensNonCapturingGroup(ParentErrorType),
    /// Error when parentheses are not properly closed
    UnclosedParens(ParentErrorType),
    /// Error when an invalid token is encountered
    InvalidTokenFound(ParentErrorType, TokenType),
    /// Error when a comma is missing in a range expression
    MissingCommaRangeExpression(ParentErrorType),
    /// Error when square brackets are missing in a range expression
    MissingSquareBracketsRangeExpression(ParentErrorType),
    /// Error when a negative group number is specified
    NegativeGroupNumber(ParentErrorType),
    /// Error when a group number is missing
    MissingGroupNumber(ParentErrorType),
    /// Error when a negative value is used in a positive reference
    NegativeValueInPositiveReference(ParentErrorType),
    /// Error when an unexpected token is encountered
    UnexpectedToken(ParentErrorType, TokenType),
    /// Error when unreachable code is executed
    UnreachableCodeReached(ParentErrorType),

    // Generator Errors
    /// Error when range values are invalid
    InvalidRangeValues(ParentErrorType, i64, i64),
    /// Error when referencing an unknown group number
    UnknownGroupNumber(ParentErrorType, u64),
}

impl fmt::Display for ClexErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[CLEX Error][{}] {}",
            self.get_parent_error_type(),
            self.get_error_message()
        )
    }
}

impl Error for ClexErrorType {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl ClexErrorType {
    fn get_error_message(&self) -> String {
        match self {
            ClexErrorType::UnclosedSingleQuotes(_) => "Expected closing single quote (') after opening single quote (')".to_string(),
            ClexErrorType::MissingColonAfterQuestionMark(_) => "Expected colon (:) after question mark (?)".to_string(),
            ClexErrorType::MissingNumberAfterNegativeSign(_) => "Expected a number after negative sign (-)".to_string(),
            ClexErrorType::NumericParsingError(_) => "Error parsing the number".to_string(),
            ClexErrorType::UnknownCharacter(_, c) => format!("Unexpected character: '{}'", c),
            ClexErrorType::UnclosedAtSymbol(_) => "Couldn't find closing @ after opening one!".to_string(),
            ClexErrorType::MissingClosingParensNonCapturingGroup(_) => "Expected closing parenthesis ')' after opening parenthesis '(' in Non-Capturing group".to_string(),
            ClexErrorType::UnclosedParens(_) => "Expected N) or ?:<UnitExpression> after opening parenthesis '('".to_string(),
            ClexErrorType::InvalidTokenFound(_, token_type) => format!("Invalid token found: {:#?}", token_type),
            ClexErrorType::InvalidCharacterSet(_) => "Invalid character set! Expected CH_UPPER, CH_LOWER, CH_ALL, CH_NUM, CH_ALPHA, CH_ALNUM, CH_NEWLINE".to_string(),

            ClexErrorType::MissingCommaRangeExpression(_) => "Expected comma (,) after opening square bracket ('[') in Range Bound Expression".to_string(),
            ClexErrorType::MissingSquareBracketsRangeExpression(_) => "Expected closing square bracket (']') after opening square bracket ('[') in Range Bound Expression".to_string(),

            ClexErrorType::NegativeGroupNumber(_) => "Group number in back-reference can't be 0 or negative!".to_string(),
            ClexErrorType::MissingGroupNumber(_) => "Expected <Group Number> after '{\\' in Quantifiers".to_string(),
            ClexErrorType::NegativeValueInPositiveReference(_) => "Literal can't be negative!".to_string(),

            ClexErrorType::UnexpectedToken(_, token_type) => format!("Expected {:?}, but not found", token_type),
            ClexErrorType::UnreachableCodeReached(_) => "Unreachable code reached!".to_string(),

            ClexErrorType::InvalidRangeValues(_, min, max) => format!("Upper bound should be greater than lower bound in [{}, {}]", min, max),
            ClexErrorType::UnknownGroupNumber(_, group_number) => format!("Can't find specified Group no. {} in the language", group_number),
        }
    }

    fn get_parent_error_type(&self) -> &'static str {
        match self {
            ClexErrorType::UnclosedSingleQuotes(parent_type)
            | ClexErrorType::MissingColonAfterQuestionMark(parent_type)
            | ClexErrorType::MissingNumberAfterNegativeSign(parent_type)
            | ClexErrorType::NumericParsingError(parent_type)
            | ClexErrorType::UnknownCharacter(parent_type, _)
            | ClexErrorType::MissingClosingParensNonCapturingGroup(parent_type)
            | ClexErrorType::UnclosedParens(parent_type)
            | ClexErrorType::InvalidTokenFound(parent_type, _)
            | ClexErrorType::MissingCommaRangeExpression(parent_type)
            | ClexErrorType::MissingSquareBracketsRangeExpression(parent_type)
            | ClexErrorType::NegativeGroupNumber(parent_type)
            | ClexErrorType::MissingGroupNumber(parent_type)
            | ClexErrorType::NegativeValueInPositiveReference(parent_type)
            | ClexErrorType::UnexpectedToken(parent_type, _)
            | ClexErrorType::UnreachableCodeReached(parent_type)
            | ClexErrorType::InvalidRangeValues(parent_type, _, _)
            | ClexErrorType::UnknownGroupNumber(parent_type, _)
            | ClexErrorType::UnclosedAtSymbol(parent_type)
            | ClexErrorType::InvalidCharacterSet(parent_type) => match parent_type {
                ParentErrorType::LexerError => "LEXER ERROR",
                ParentErrorType::ParserError => "PARSER ERROR",
                ParentErrorType::GeneratorError => "GENERATOR ERROR",
            },
        }
    }
}
