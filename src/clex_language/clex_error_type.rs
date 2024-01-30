use crate::clex_language::lexer::TokenType;
use std::process::exit;

#[derive(Debug)]

pub enum ParentErrorType {
    LexerError,
    ParserError,
    GeneratorError,
}

#[derive(Debug)]
pub enum ClexErrorType {
    /// Lexical Errors
    UnclosedSingleQuotes(ParentErrorType),
    MissingColonAfterQuestionMark(ParentErrorType),
    MissingNumberAfterNegativeSign(ParentErrorType),
    NumericParsingError(ParentErrorType),
    UnknownCharacter(ParentErrorType, &'static str),

    /// Parser Errors
    MissingClosingParensCapturingGroup(ParentErrorType),
    MissingClosingParensNonCapturingGroup(ParentErrorType),
    UnclosedParens(ParentErrorType),
    InvalidTokenFound(ParentErrorType, TokenType),

    MissingClosingBracesQuantifiers(ParentErrorType),
    MissingCommaStringModifierExpression(ParentErrorType),
    MissingSquareBracketsStringModifierExpression(ParentErrorType),
    MissingCommaRangeExpression(ParentErrorType),
    MissingSquareBracketsRangeExpression(ParentErrorType),

    NegativeGroupNumber(ParentErrorType),
    MissingGroupNumber(ParentErrorType),
    NegativeValueInPositiveReference(ParentErrorType),

    UnexpectedToken(ParentErrorType, TokenType),
    UnreachableCodeReached(ParentErrorType),

    /// Generator Error
    InvalidRangeValues(ParentErrorType, i64, i64),
    UnknownGroupNumber(ParentErrorType, u64),
}

impl ClexErrorType {
    pub fn print_and_exit(&self) {
        eprintln!("{:?} {}", self, self.get_msg());
        exit(1);
    }
    pub fn get_msg(&self) -> String {
        match self {
            ClexErrorType::UnclosedSingleQuotes(_) => {
                "Expected closing ' after opening '".to_string()
            }
            ClexErrorType::MissingColonAfterQuestionMark(_) => "Expected : after ?".to_string(),
            ClexErrorType::MissingNumberAfterNegativeSign(_) => {
                "Expected Number after -".to_string()
            }
            ClexErrorType::NumericParsingError(_) => "Error parsing the number".to_string(),
            ClexErrorType::UnknownCharacter(_, c) => {
                format!("Unexpected character {c}")
            }
            ClexErrorType::MissingClosingParensCapturingGroup(_) => {
                "Expected ) after (N in Capturing Group".to_string()
            }
            ClexErrorType::MissingClosingParensNonCapturingGroup(_) => {
                "Expected a closing parens ) after (:? in non-capturing group".to_string()
            }
            ClexErrorType::UnclosedParens(_) => {
                "Expected N) or ?:<UnitExpression> after opening (".to_string()
            }
            ClexErrorType::InvalidTokenFound(_, token_type) => {
                format!("Invalid Token found! {:#?}", token_type)
            }
            ClexErrorType::MissingClosingBracesQuantifiers(_) => {
                "Expected }} after {{\\N in Quantifiers".to_string()
            }
            ClexErrorType::MissingCommaStringModifierExpression(_) => {
                "Expected , after [ in String Modifier Expression".to_string()
            }
            ClexErrorType::MissingSquareBracketsStringModifierExpression(_) => {
                "Expected character or ] after [ in String Modifier Expression".to_string()
            }
            ClexErrorType::MissingCommaRangeExpression(_) => {
                "Expected , after [ in Range Bound Expression".to_string()
            }
            ClexErrorType::MissingSquareBracketsRangeExpression(_) => {
                "Expected ] after [ in Range Bound Expression".to_string()
            }
            ClexErrorType::NegativeGroupNumber(_) => {
                "Group Number in Back-reference can't be 0 or negative!".to_string()
            }
            ClexErrorType::MissingGroupNumber(_) => {
                "Expected <Group Number> after {{\\ in Quantifiers".to_string()
            }
            ClexErrorType::NegativeValueInPositiveReference(_) => {
                "Literal can't be negative!".to_string()
            }
            ClexErrorType::UnexpectedToken(_, token_type) => {
                format!("Expected {token_type:?}, but not found!")
            }
            ClexErrorType::UnreachableCodeReached(_) => "Unreachable code reached!".to_string(),
            ClexErrorType::InvalidRangeValues(_, min, max) => {
                format!("Upper bound should be greater than lower bound in [m({min}),n({max})]")
            }
            ClexErrorType::UnknownGroupNumber(_, group_number) => {
                format!("Can't find specified Group no. {group_number} in the language")
            }
        }
    }
}
