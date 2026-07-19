//! The `parser` module is responsible for analyzing and converting tokenized input into an Abstract Syntax Tree (AST)
//! representation following the `clex` language grammar rules.
//!
//! The Parser takes a sequence of tokens produced by the lexer and constructs a structured AST that represents
//! the hierarchical relationships between different language constructs. It handles various expressions including:
//!
//! - Primitive expressions (Integer, Float, String)
//! - Capturing groups with ranges
//! - Non-capturing groups with nested expressions
//! - Quantifiers and modifiers
//! - Character sets for strings
//!
//! # Examples
//!
//! ```rust
//! use clex_gen::clex_language::parser::Parser;
//!
//! let source = "N[1,100]";
//! let mut parser = Parser::new(source.to_string()).unwrap();
//! parser.parser().unwrap();
//! let ast = parser.get_language();
//! ```
//!
//! The parser performs recursive descent parsing and maintains state about:
//! - Current token position
//! - Number of capturing groups
//! - The AST being constructed
//!
//! It provides detailed error reporting for syntax violations and invalid constructs.

use super::lexer::{TokenType, Tokens};
use crate::clex_language;
use crate::clex_language::ast::{
    CharacterSet, ClexLanguageAST, DataType, FloatReferenceType, PositiveReferenceType,
    ReferenceType, Repetition, UnitExpression,
};
use crate::clex_language::clex_error_type::{ClexErrorType, ParentErrorType};
use crate::clex_language::lexer::Token;

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Tokens,
    start: usize,
    current: usize,
    language: ClexLanguageAST,
    current_group: u64, // for capturing groupCount, starts from 1.....
}

impl Parser {
    pub fn new(source_language: String) -> Result<Self, ClexErrorType> {
        let mut tokens = Tokens::new(source_language);
        tokens.scan_tokens()?;

        Ok(Self {
            tokens,
            start: 0,
            current: 0,
            language: ClexLanguageAST { expression: vec![] },
            current_group: 0,
        })
    }

    pub fn get_language(&self) -> &ClexLanguageAST {
        &self.language
    }

    pub fn new_from_tokens(tokens: Tokens) -> Self {
        Self {
            tokens,
            start: 0,
            current: 0,
            language: ClexLanguageAST { expression: vec![] },
            current_group: 0,
        }
    }

    pub fn parser(&mut self) -> Result<(), ClexErrorType> {
        while !self.at_end() {
            self.start = self.current;
            let expr = self.parse_expr()?;
            self.language.expression.push(expr);
        }

        Ok(())
    }

    fn parse_expr(&mut self) -> Result<UnitExpression, ClexErrorType> {
        let token = self.advance();

        match &token.token_type {
            TokenType::Integer | TokenType::String | TokenType::Float => {
                let dt = token.token_type.clone();
                self.parse_primitive_expr(dt, token.span)
            }
            TokenType::LeftParens => self.parse_group_expr(token.span),
            TokenType::LiteralText(text) => Ok(UnitExpression::Literal {
                text: text.clone(),
                span: token.span,
            }),
            TokenType::Eof => Ok(UnitExpression::Eof),
            _ => Err(ClexErrorType::InvalidTokenFound(
                ParentErrorType::ParserError,
                token.span,
                token.token_type.clone(),
            )),
        }
    }

    fn parse_primitive_expr(
        &mut self,
        data_type: TokenType,
        start_span: crate::clex_language::lexer::Span,
    ) -> Result<UnitExpression, ClexErrorType> {
        match data_type {
            TokenType::Integer => {
                let (lower_bound, upper_bound) = self.parse_range()?;
                let repetition_type = self.parse_quantifier()?;

                Ok(UnitExpression::Primitives {
                    data_type: DataType::Integer(lower_bound, upper_bound),
                    repetition: repetition_type,
                    span: start_span,
                })
            }
            TokenType::Float => {
                let (lower_reference, upper_reference) = self.parse_float_range()?;
                let repetition_type = self.parse_quantifier()?;

                Ok(UnitExpression::Primitives {
                    data_type: DataType::Float(lower_reference, upper_reference),
                    repetition: repetition_type,
                    span: start_span,
                })
            }
            TokenType::String => {
                let (min_length, max_length, charset) = self.parse_string_modifiers()?;
                let repetition_type = self.parse_quantifier()?;

                Ok(UnitExpression::Primitives {
                    data_type: DataType::String(min_length, max_length, charset),
                    repetition: repetition_type,
                    span: start_span,
                })
            }
            _ => Err(ClexErrorType::UnreachableCodeReached(
                ParentErrorType::ParserError,
                start_span,
            )),
        }
    }

    fn parse_group_expr(
        &mut self,
        start_span: crate::clex_language::lexer::Span,
    ) -> Result<UnitExpression, ClexErrorType> {
        if self.match_token(&TokenType::Integer) {
            let (lower_reference, upper_reference) = self.parse_positive_range()?;
            self.expect(&TokenType::RightParens)?;

            self.current_group += 1;

            Ok(UnitExpression::CapturingGroup {
                group_number: self.current_group,
                range: (lower_reference, upper_reference),
                span: start_span,
            })
        } else if self.match_token(&TokenType::QuestionColon) {
            let last_index = self
                .peek_from_current(TokenType::RightParens, TokenType::LeftParens)
                .ok_or(ClexErrorType::MissingClosingParensNonCapturingGroup(
                    ParentErrorType::ParserError,
                    start_span,
                ))?;

            let mut branches = vec![Vec::new()];

            while self.current < last_index {
                if self.match_token(&TokenType::Pipe) {
                    branches.push(Vec::new());
                    continue;
                }
                let expr = self.parse_expr()?;
                match expr {
                    UnitExpression::Primitives { .. }
                    | UnitExpression::NonCapturingGroup { .. }
                    | UnitExpression::CapturingGroup { .. }
                    | UnitExpression::Literal { .. } => {
                        if let Some(current_branch) = branches.last_mut() {
                            current_branch.push(expr);
                        }
                    }
                    UnitExpression::Eof => break,
                }
            }

            self.expect(&TokenType::RightParens)?;

            let repetition_type = self.parse_quantifier()?;
            Ok(UnitExpression::NonCapturingGroup {
                branches,
                repetition: repetition_type,
                span: start_span,
            })
        } else {
            Err(ClexErrorType::UnclosedParens(
                ParentErrorType::ParserError,
                start_span,
            ))
        }
    }

    fn parse_quantifier(&mut self) -> Result<Repetition, ClexErrorType> {
        if self.match_token(&TokenType::LeftCurlyBrackets) {
            let first_reference =
                self.parse_positive_reference(clex_language::ast::DEFAULT_QUANTIFIER_VALUE)?;

            if self.match_token(&TokenType::Comma) {
                let second_reference =
                    self.parse_positive_reference(clex_language::ast::DEFAULT_QUANTIFIER_VALUE)?;
                self.expect(&TokenType::RightCurlyBrackets)?;
                Ok(Repetition::Range(first_reference, second_reference))
            } else {
                self.expect(&TokenType::RightCurlyBrackets)?;
                Ok(Repetition::Exact(first_reference))
            }
        } else {
            Ok(Repetition::Exact(PositiveReferenceType::ByLiteral(
                clex_language::ast::DEFAULT_QUANTIFIER_VALUE,
            )))
        }
    }

    fn try_parse_character_set(&mut self) -> Option<CharacterSet> {
        let cs = match &self.peek().token_type {
            TokenType::LiteralString(charset) => Some(CharacterSet::Custom(charset.to_string())),
            TokenType::CharacterSetAlpha => Some(CharacterSet::Alphabet),
            TokenType::CharacterSetAlnum => Some(CharacterSet::AlphaNumeric),
            TokenType::CharacterSetNewline => Some(CharacterSet::Newline),
            TokenType::CharacterSetNumeric => Some(CharacterSet::Numeric),
            TokenType::CharacterSetUpper => Some(CharacterSet::Uppercase),
            TokenType::CharacterSetLower => Some(CharacterSet::LowerCase),
            TokenType::CharacterSetAll => Some(CharacterSet::All),
            _ => None,
        };
        if cs.is_some() {
            self.advance();
        }
        cs
    }

    fn parse_string_modifiers(
        &mut self,
    ) -> Result<(PositiveReferenceType, PositiveReferenceType, CharacterSet), ClexErrorType> {
        let mut min_length_reference =
            PositiveReferenceType::ByLiteral(clex_language::ast::DEFAULT_MIN_STRING_SIZE as u64);
        let mut max_length_reference =
            PositiveReferenceType::ByLiteral(clex_language::ast::DEFAULT_MAX_STRING_SIZE as u64);
        let mut char_set = CharacterSet::get_default_charset();

        if self.match_token(&TokenType::LeftSquareBracket) {
            let first_ref =
                self.parse_positive_reference(clex_language::ast::DEFAULT_MIN_STRING_SIZE as u64)?;

            if self.match_token(&TokenType::Comma) {
                if let Some(cs) = self.try_parse_character_set() {
                    // S[len, charset]
                    min_length_reference = first_ref.clone();
                    max_length_reference = first_ref;
                    char_set = cs;
                } else {
                    // S[min, max] or S[min, max, charset]
                    min_length_reference = first_ref;
                    max_length_reference = self.parse_positive_reference(
                        clex_language::ast::DEFAULT_MAX_STRING_SIZE as u64,
                    )?;

                    if self.match_token(&TokenType::Comma) {
                        if let Some(cs) = self.try_parse_character_set() {
                            char_set = cs;
                        }
                    }
                }
            } else {
                // S[length]
                min_length_reference = first_ref.clone();
                max_length_reference = first_ref;
            }

            self.expect(&TokenType::RightSquareBracket)?;
        }

        Ok((min_length_reference, max_length_reference, char_set))
    }

    fn parse_range(&mut self) -> Result<(ReferenceType, ReferenceType), ClexErrorType> {
        let lower_bound = clex_language::ast::DEFAULT_RANGE_MIN_VALUE;
        let upper_bound = clex_language::ast::DEFAULT_RANGE_MAX_VALUE;
        let mut lower_reference = ReferenceType::ByLiteral(lower_bound);
        let mut upper_reference = ReferenceType::ByLiteral(upper_bound);

        if self.match_token(&TokenType::LeftSquareBracket) {
            let bracket_span = if self.current > 0 {
                self.tokens.get_tokens()[self.current - 1].span
            } else {
                crate::clex_language::lexer::Span { start: 0, end: 0 }
            };

            lower_reference = self.parse_reference(lower_bound)?;

            if !self.match_token(&TokenType::Comma) {
                let current_span = if self.current < self.tokens.get_tokens().len() {
                    self.tokens.get_tokens()[self.current].span
                } else {
                    bracket_span
                };
                return Err(ClexErrorType::MissingCommaRangeExpression(
                    ParentErrorType::ParserError,
                    current_span,
                ));
            }

            upper_reference = self.parse_reference(upper_bound)?;

            if !self.match_token(&TokenType::RightSquareBracket) {
                let current_span = if self.current < self.tokens.get_tokens().len() {
                    self.tokens.get_tokens()[self.current].span
                } else {
                    bracket_span
                };
                return Err(ClexErrorType::MissingSquareBracketsRangeExpression(
                    ParentErrorType::ParserError,
                    current_span,
                ));
            }
        }

        Ok((lower_reference, upper_reference))
    }

    fn parse_float_range(
        &mut self,
    ) -> Result<(FloatReferenceType, FloatReferenceType), ClexErrorType> {
        let lower_bound = clex_language::ast::DEFAULT_RANGE_MIN_VALUE as f64;
        let upper_bound = clex_language::ast::DEFAULT_RANGE_MAX_VALUE as f64;
        let mut lower_reference = FloatReferenceType::ByLiteral(lower_bound);
        let mut upper_reference = FloatReferenceType::ByLiteral(upper_bound);

        if self.match_token(&TokenType::LeftSquareBracket) {
            let bracket_span = if self.current > 0 {
                self.tokens.get_tokens()[self.current - 1].span
            } else {
                crate::clex_language::lexer::Span { start: 0, end: 0 }
            };

            lower_reference = self.parse_float_reference(lower_bound)?;

            if !self.match_token(&TokenType::Comma) {
                let current_span = if self.current < self.tokens.get_tokens().len() {
                    self.tokens.get_tokens()[self.current].span
                } else {
                    bracket_span
                };
                return Err(ClexErrorType::MissingCommaRangeExpression(
                    ParentErrorType::ParserError,
                    current_span,
                ));
            }

            upper_reference = self.parse_float_reference(upper_bound)?;

            if !self.match_token(&TokenType::RightSquareBracket) {
                let current_span = if self.current < self.tokens.get_tokens().len() {
                    self.tokens.get_tokens()[self.current].span
                } else {
                    bracket_span
                };
                return Err(ClexErrorType::MissingSquareBracketsRangeExpression(
                    ParentErrorType::ParserError,
                    current_span,
                ));
            }
        }

        Ok((lower_reference, upper_reference))
    }

    fn parse_positive_range(
        &mut self,
    ) -> Result<(PositiveReferenceType, PositiveReferenceType), ClexErrorType> {
        let lower_bound = clex_language::ast::DEFAULT_POSITIVE_RANGE_MIN_VALUE;
        let upper_bound = clex_language::ast::DEFAULT_POSITIVE_RANGE_MAX_VALUE;
        let mut lower_reference = PositiveReferenceType::ByLiteral(lower_bound);
        let mut upper_reference = PositiveReferenceType::ByLiteral(upper_bound);

        if self.match_token(&TokenType::LeftSquareBracket) {
            let bracket_span = if self.current > 0 {
                self.tokens.get_tokens()[self.current - 1].span
            } else {
                crate::clex_language::lexer::Span { start: 0, end: 0 }
            };

            lower_reference = self.parse_positive_reference(lower_bound)?;

            if !self.match_token(&TokenType::Comma) {
                let current_span = if self.current < self.tokens.get_tokens().len() {
                    self.tokens.get_tokens()[self.current].span
                } else {
                    bracket_span
                };
                return Err(ClexErrorType::MissingCommaRangeExpression(
                    ParentErrorType::ParserError,
                    current_span,
                ));
            }

            upper_reference = self.parse_positive_reference(upper_bound)?;

            if !self.match_token(&TokenType::RightSquareBracket) {
                let current_span = if self.current < self.tokens.get_tokens().len() {
                    self.tokens.get_tokens()[self.current].span
                } else {
                    bracket_span
                };
                return Err(ClexErrorType::MissingSquareBracketsRangeExpression(
                    ParentErrorType::ParserError,
                    current_span,
                ));
            }
        }

        Ok((lower_reference, upper_reference))
    }

    fn parse_positive_reference(
        &mut self,
        default_value: u64,
    ) -> Result<PositiveReferenceType, ClexErrorType> {
        if self.match_token(&TokenType::Backslash) {
            let backslash_span = if self.current > 0 {
                self.tokens.get_tokens()[self.current - 1].span
            } else {
                crate::clex_language::lexer::Span { start: 0, end: 0 }
            };

            if let TokenType::LiteralNumber(value) = self.peek().token_type {
                let num_span = self.peek().span;
                self.advance();
                if value <= 0 {
                    Err(ClexErrorType::NegativeGroupNumber(
                        ParentErrorType::ParserError,
                        num_span,
                    ))
                } else if value as u64 > self.current_group {
                    Err(ClexErrorType::UnknownGroupNumber(
                        ParentErrorType::ParserError,
                        num_span,
                        value as u64,
                    ))
                } else {
                    Ok(PositiveReferenceType::ByGroup {
                        group_number: value as u64,
                    })
                }
            } else {
                Err(ClexErrorType::MissingGroupNumber(
                    ParentErrorType::ParserError,
                    backslash_span,
                ))
            }
        } else if let TokenType::LiteralNumber(value) = self.peek().token_type {
            let num_span = self.peek().span;
            self.advance();
            if value < 0 {
                Err(ClexErrorType::NegativeValueInPositiveReference(
                    ParentErrorType::ParserError,
                    num_span,
                ))
            } else {
                Ok(PositiveReferenceType::ByLiteral(value as u64))
            }
        } else {
            Ok(PositiveReferenceType::ByLiteral(default_value))
        }
    }

    fn parse_reference(&mut self, default_value: i64) -> Result<ReferenceType, ClexErrorType> {
        if self.match_token(&TokenType::Backslash) {
            let backslash_span = if self.current > 0 {
                self.tokens.get_tokens()[self.current - 1].span
            } else {
                crate::clex_language::lexer::Span { start: 0, end: 0 }
            };

            if let TokenType::LiteralNumber(value) = self.peek().token_type {
                let num_span = self.peek().span;
                self.advance();
                if value <= 0 {
                    Err(ClexErrorType::NegativeGroupNumber(
                        ParentErrorType::ParserError,
                        num_span,
                    ))
                } else if value as u64 > self.current_group {
                    Err(ClexErrorType::UnknownGroupNumber(
                        ParentErrorType::ParserError,
                        num_span,
                        value as u64,
                    ))
                } else {
                    Ok(ReferenceType::ByGroup {
                        group_number: value as u64,
                    })
                }
            } else {
                Err(ClexErrorType::MissingGroupNumber(
                    ParentErrorType::ParserError,
                    backslash_span,
                ))
            }
        } else if let TokenType::LiteralNumber(value) = self.peek().token_type {
            self.advance();
            Ok(ReferenceType::ByLiteral(value))
        } else {
            Ok(ReferenceType::ByLiteral(default_value))
        }
    }

    fn parse_float_reference(
        &mut self,
        default_value: f64,
    ) -> Result<FloatReferenceType, ClexErrorType> {
        if self.match_token(&TokenType::Backslash) {
            let backslash_span = if self.current > 0 {
                self.tokens.get_tokens()[self.current - 1].span
            } else {
                crate::clex_language::lexer::Span { start: 0, end: 0 }
            };

            if let TokenType::LiteralNumber(value) = self.peek().token_type {
                let num_span = self.peek().span;
                self.advance();
                if value <= 0 {
                    Err(ClexErrorType::NegativeGroupNumber(
                        ParentErrorType::ParserError,
                        num_span,
                    ))
                } else if value as u64 > self.current_group {
                    Err(ClexErrorType::UnknownGroupNumber(
                        ParentErrorType::ParserError,
                        num_span,
                        value as u64,
                    ))
                } else {
                    Ok(FloatReferenceType::ByGroup {
                        group_number: value as u64,
                    })
                }
            } else {
                Err(ClexErrorType::MissingGroupNumber(
                    ParentErrorType::ParserError,
                    backslash_span,
                ))
            }
        } else if let TokenType::LiteralFloat(value) = self.peek().token_type {
            self.advance();
            Ok(FloatReferenceType::ByLiteral(value))
        } else if let TokenType::LiteralNumber(value) = self.peek().token_type {
            self.advance();
            Ok(FloatReferenceType::ByLiteral(value as f64))
        } else {
            Ok(FloatReferenceType::ByLiteral(default_value))
        }
    }

    fn peek_from_current(&mut self, expected: TokenType, not_expected: TokenType) -> Option<usize> {
        let mut depth = 0;
        let current_reset_duplicate = self.current;

        while !self.at_end() {
            let tk = self.advance();

            if tk.token_type == not_expected {
                depth += 1;
            } else if tk.token_type == expected {
                if depth == 0 {
                    let expected_index = self.current - 1;
                    self.current = current_reset_duplicate;
                    return Some(expected_index);
                } else {
                    depth -= 1;
                }
            }
        }

        self.current = current_reset_duplicate;
        None
    }

    fn expect(&mut self, expected: &TokenType) -> Result<(), ClexErrorType> {
        if !self.match_token(expected) {
            let current_span = if self.current < self.tokens.get_tokens().len() {
                self.tokens.get_tokens()[self.current].span
            } else if self.current > 0 {
                self.tokens.get_tokens()[self.current - 1].span
            } else {
                crate::clex_language::lexer::Span { start: 0, end: 0 }
            };
            Err(ClexErrorType::UnexpectedToken(
                ParentErrorType::ParserError,
                current_span,
                expected.clone(),
            ))
        } else {
            Ok(())
        }
    }

    fn advance(&mut self) -> Token {
        self.current += 1;
        self.tokens.get_tokens()[self.current - 1].clone()
    }

    fn peek(&mut self) -> Token {
        if self.at_end() {
            Token {
                token_type: TokenType::Eof,
                lexeme: String::new(),
                span: crate::clex_language::lexer::Span {
                    start: self.current,
                    end: self.current,
                },
            }
        } else {
            self.tokens.get_tokens()[self.current].clone()
        }
    }

    fn match_token(&mut self, expected: &TokenType) -> bool {
        // Move forward if expected token is present
        if self.at_end() || &self.tokens.get_tokens()[self.current].token_type != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn at_end(&mut self) -> bool {
        self.current >= self.tokens.get_tokens().len()
    }
}
