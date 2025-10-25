//! Error handling module for the clex language implementation.
//!
//! This module provides error types and implementations for handling various errors
//! that can occur during lexical analysis, parsing, and code generation phases.

use crate::clex_language::lexer::{Span, TokenType};
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
    UnclosedSingleQuotes(ParentErrorType, Span),
    /// Error when a colon is missing after a question mark
    MissingColonAfterQuestionMark(ParentErrorType, Span),
    /// Error when a negative sign is not followed by a number
    MissingNumberAfterNegativeSign(ParentErrorType, Span),
    /// Error when parsing numeric values
    NumericParsingError(ParentErrorType, Span),
    /// Error when encountering an unknown character
    UnknownCharacter(ParentErrorType, Span, &'static str),
    /// Error when an @ symbol is not properly closed
    UnclosedAtSymbol(ParentErrorType, Span),
    /// Error when an invalid character set is specified, all valid character sets are specified in <https://github.com/rootCircle/cpast_mono/blob/main/clex_gen/docs/CLEX_LANG_SPECS.md#character>
    InvalidCharacterSet(ParentErrorType, Span),

    // Parser Errors
    /// Error when a non-capturing group is missing closing parenthesis
    MissingClosingParensNonCapturingGroup(ParentErrorType, Span),
    /// Error when parentheses are not properly closed
    UnclosedParens(ParentErrorType, Span),
    /// Error when an invalid token is encountered
    InvalidTokenFound(ParentErrorType, Span, TokenType),
    /// Error when a comma is missing in a range expression
    MissingCommaRangeExpression(ParentErrorType, Span),
    /// Error when square brackets are missing in a range expression
    MissingSquareBracketsRangeExpression(ParentErrorType, Span),
    /// Error when a negative group number is specified
    NegativeGroupNumber(ParentErrorType, Span),
    /// Error when a group number is missing
    MissingGroupNumber(ParentErrorType, Span),
    /// Error when a negative value is used in a positive reference
    NegativeValueInPositiveReference(ParentErrorType, Span),
    /// Error when an unexpected token is encountered
    UnexpectedToken(ParentErrorType, Span, TokenType),
    /// Error when unreachable code is executed
    UnreachableCodeReached(ParentErrorType, Span),

    // Generator Errors
    /// Error when range values are invalid
    InvalidRangeValues(ParentErrorType, Span, i64, i64),
    /// Error when referencing an unknown group number
    UnknownGroupNumber(ParentErrorType, Span, u64),
}

impl fmt::Display for ClexErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let span = self.get_span();
        write!(
            f,
            "[CLEX Error][{}] {} at position {}..{}",
            self.get_parent_error_type(),
            self.get_error_message(),
            span.start,
            span.end
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
            ClexErrorType::UnclosedSingleQuotes(_, _) => "Expected closing single quote (') after opening single quote (')".to_string(),
            ClexErrorType::MissingColonAfterQuestionMark(_, _) => "Expected colon (:) after question mark (?)".to_string(),
            ClexErrorType::MissingNumberAfterNegativeSign(_, _) => "Expected a number after negative sign (-)".to_string(),
            ClexErrorType::NumericParsingError(_, _) => "Error parsing the number".to_string(),
            ClexErrorType::UnknownCharacter(_, _, c) => format!("Unexpected character: '{c}'"),
            ClexErrorType::UnclosedAtSymbol(_, _) => "Couldn't find closing @ after opening one!".to_string(),
            ClexErrorType::MissingClosingParensNonCapturingGroup(_, _) => "Expected closing parenthesis ')' after opening parenthesis '(' in Non-Capturing group".to_string(),
            ClexErrorType::UnclosedParens(_, _) => "Expected N) or ?:<UnitExpression> after opening parenthesis '('".to_string(),
            ClexErrorType::InvalidTokenFound(_, _, token_type) => format!("Invalid token found: {token_type:#?}"),
            ClexErrorType::InvalidCharacterSet(_, _) => "Invalid character set! Expected CH_UPPER, CH_LOWER, CH_ALL, CH_NUM, CH_ALPHA, CH_ALNUM, CH_NEWLINE".to_string(),

            ClexErrorType::MissingCommaRangeExpression(_, _) => "Expected comma (,) after opening square bracket ('[') in Range Bound Expression".to_string(),
            ClexErrorType::MissingSquareBracketsRangeExpression(_, _) => "Expected closing square bracket (']') after opening square bracket ('[') in Range Bound Expression".to_string(),

            ClexErrorType::NegativeGroupNumber(_, _) => "Group number in back-reference can't be 0 or negative!".to_string(),
            ClexErrorType::MissingGroupNumber(_, _) => "Expected <Group Number> after '{\\' in Quantifiers".to_string(),
            ClexErrorType::NegativeValueInPositiveReference(_, _) => "Literal can't be negative!".to_string(),

            ClexErrorType::UnexpectedToken(_, _, token_type) => format!("Expected {token_type:?}, but not found"),
            ClexErrorType::UnreachableCodeReached(_, _) => "Unreachable code reached!".to_string(),

            ClexErrorType::InvalidRangeValues(_, _, min, max) => format!("Upper bound should be greater than lower bound in [{min}, {max}]"),
            ClexErrorType::UnknownGroupNumber(_, _, group_number) => format!("Can't find specified Group no. {group_number} in the language"),
        }
    }

    fn get_span(&self) -> Span {
        match self {
            ClexErrorType::UnclosedSingleQuotes(_, span)
            | ClexErrorType::MissingColonAfterQuestionMark(_, span)
            | ClexErrorType::MissingNumberAfterNegativeSign(_, span)
            | ClexErrorType::NumericParsingError(_, span)
            | ClexErrorType::UnknownCharacter(_, span, _)
            | ClexErrorType::MissingClosingParensNonCapturingGroup(_, span)
            | ClexErrorType::UnclosedParens(_, span)
            | ClexErrorType::InvalidTokenFound(_, span, _)
            | ClexErrorType::MissingCommaRangeExpression(_, span)
            | ClexErrorType::MissingSquareBracketsRangeExpression(_, span)
            | ClexErrorType::NegativeGroupNumber(_, span)
            | ClexErrorType::MissingGroupNumber(_, span)
            | ClexErrorType::NegativeValueInPositiveReference(_, span)
            | ClexErrorType::UnexpectedToken(_, span, _)
            | ClexErrorType::UnreachableCodeReached(_, span)
            | ClexErrorType::InvalidRangeValues(_, span, _, _)
            | ClexErrorType::UnknownGroupNumber(_, span, _)
            | ClexErrorType::UnclosedAtSymbol(_, span)
            | ClexErrorType::InvalidCharacterSet(_, span) => *span,
        }
    }

    fn get_parent_error_type(&self) -> &'static str {
        match self {
            ClexErrorType::UnclosedSingleQuotes(parent_type, _)
            | ClexErrorType::MissingColonAfterQuestionMark(parent_type, _)
            | ClexErrorType::MissingNumberAfterNegativeSign(parent_type, _)
            | ClexErrorType::NumericParsingError(parent_type, _)
            | ClexErrorType::UnknownCharacter(parent_type, _, _)
            | ClexErrorType::MissingClosingParensNonCapturingGroup(parent_type, _)
            | ClexErrorType::UnclosedParens(parent_type, _)
            | ClexErrorType::InvalidTokenFound(parent_type, _, _)
            | ClexErrorType::MissingCommaRangeExpression(parent_type, _)
            | ClexErrorType::MissingSquareBracketsRangeExpression(parent_type, _)
            | ClexErrorType::NegativeGroupNumber(parent_type, _)
            | ClexErrorType::MissingGroupNumber(parent_type, _)
            | ClexErrorType::NegativeValueInPositiveReference(parent_type, _)
            | ClexErrorType::UnexpectedToken(parent_type, _, _)
            | ClexErrorType::UnreachableCodeReached(parent_type, _)
            | ClexErrorType::InvalidRangeValues(parent_type, _, _, _)
            | ClexErrorType::UnknownGroupNumber(parent_type, _, _)
            | ClexErrorType::UnclosedAtSymbol(parent_type, _)
            | ClexErrorType::InvalidCharacterSet(parent_type, _) => match parent_type {
                ParentErrorType::LexerError => "LEXER ERROR",
                ParentErrorType::ParserError => "PARSER ERROR",
                ParentErrorType::GeneratorError => "GENERATOR ERROR",
            },
        }
    }
}
