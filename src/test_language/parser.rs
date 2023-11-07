use std::process::exit;
use crate::test_language::ast::{AstLanguage, DataType, Expression, NumeralDataType, UnitExpression};
use crate::test_language::lexer::Token;
use super::lexer::{Tokens, TokenType};

pub(crate) struct Parser {
    tokens: Tokens,
    start: usize,
    current: usize,
    language: AstLanguage,
    groups: u64 // for capturing groups, starts from 1.....
}

impl Parser {
    pub fn new(source_language: String) -> Self {
        Parser {
            tokens: Tokens::new(source_language),
            start: 0,
            current: 0,
            language: AstLanguage {
                tokens: vec![]
            },
            groups: 0
        }
    }

    pub fn parser(&mut self, tokens: Tokens) -> AstLanguage{

        while !self.at_end() {
            let mut expr = self.parse_expr();
            self.language.tokens.push(expr);
        }

        AstLanguage { tokens: vec![] }
    }

    pub fn parse_expr(&mut self) -> Expression {
        let mut unit = vec![
            self.parse_range_bound_primitives()
            .or(self.parse_primary())
            .or(self.parse_capturing_group())
            .or(self.parse_non_capturing_group()).unwrap()
        ];

        Expression { unit }
    }

    fn parse_sub_expression(&mut self, start: usize, end: usize) -> Expression {
        //! @TODO
        Expression { unit: vec![] }
    }
    
    fn parse_unit_expression(&mut self) -> UnitExpression {
        //! @TODO
        UnitExpression::Primitives { data_type: DataType::Integer }
    }

    fn parse_repeating_expressions(&mut self) -> Option<UnitExpression> {
        Some(UnitExpression::Primitives { data_type: DataType::Integer })
    }

    fn parse_range_bound_primitives(&mut self) -> Option<UnitExpression> {
        let tk = self.advance();
        
        if (tk.token_type ==  TokenType::Integer || tk.token_type == TokenType::Float) && self.match_token(TokenType::LeftSquareBracket) {

            let mut lower_bound = i64 ::MIN;
            let mut upper_bound = i64 ::MAX;

            if let TokenType::LiteralNumber(lower) = self.peek().token_type {
                self.current += 1;
                lower_bound = lower;
            }

            if !self.match_token(TokenType::Comma) {
                eprintln!("Expected , after [ in Range Bound Expression");
                exit(1);
            }

            if let TokenType::LiteralNumber(upper) = self.peek().token_type {
                self.current += 1;
                upper_bound = upper;
            }

            if !self.match_token(TokenType::RightSquareBracket) {
                eprintln!("Expected ] after [ in Range Bound Expression");
                exit(1);
            }

            return Some(UnitExpression::RangeBoundPrimitives {
                data_type: match tk.token_type {
                    TokenType::Integer => NumeralDataType::Integer,
                    TokenType::Float => NumeralDataType::Float,
                    _ => {
                        eprintln!("Unknown Token Type!");
                        exit(1);
                    }
                },
                lower_bound,
                upper_bound,
            });
        }
       None
    }

    fn parse_non_capturing_group(&mut self) -> Option<UnitExpression> {
        let tk = self.advance();

        if tk.token_type == TokenType::LeftParens && self.match_token(TokenType::QuestionColon) {
            let next_paren: usize = match self.peek_from_current(TokenType::RightParens, TokenType::LeftParens) {
                ..=-1 => {
                    eprintln!("Expected closing ) after (?:");
                    exit(1);
                },
                t => {
                    t as usize
                }
            };

            // let sub_token = Parser::new(
            //
            // );
            return Some(UnitExpression::NonCapturingGroup {
                nest_exp: self.parse_sub_expression(self.current, next_paren),
            });

        }
        None
    }

    fn parse_capturing_group(&mut self) -> Option<UnitExpression> {
        let tk = self.advance();

        if tk.token_type == TokenType::LeftParens {
            if self.match_token(TokenType::Integer) {
                if self.match_token(TokenType::RightParens) {
                    self.groups += 1;
                    return Some(UnitExpression::CapturingGroup {
                        group_number: self.groups,
                    });
                }
                else {
                    eprintln!("Expected ) after (N");
                    exit(1);
                }
            }
            else if self.match_token(TokenType::QuestionColon) {
                self.current -= 2; // Move back twice for eaten ( ?: i.e., LeftParens QuestionColon
                return None;
            }
            else {
                eprintln!("Unexpected character after (\n Expecting (N) or (?:");
                exit(1);
            }
        }
        None
    }

    fn parse_primary(&mut self) -> Option<UnitExpression> {
        let tk = self.advance();

        match tk.token_type {
            TokenType::Integer => Some(UnitExpression::Primitives {data_type: DataType::Integer}),
            TokenType::Float => Some(UnitExpression::Primitives {data_type: DataType::Float}),
            TokenType::String => Some(UnitExpression::Primitives {data_type: DataType::String}),
            TokenType::Character => Some(UnitExpression::Primitives {data_type: DataType::Character}),
            _ => None
        }
    }

    fn peek_from_current(&mut self, expected: TokenType, not_expected: TokenType) -> i64 {
        // Finds index of occurrence of expected Token from current position
        // while self.lookup(expected, self.current) 
        -1
    }

    fn lookup(&mut self, expected: TokenType, current: usize) -> i64 {
        // Finds index of occurrence of expected Token from current position
        if let Some(index) = self.tokens.tokens.iter().skip(current).position(|item| item.token_type == expected) {
            return (index + current) as i64;
        }
        -1
    }

    fn advance(&mut self) -> Token {
        self.current += 1;
        self.tokens.tokens[self.current - 1].clone()
    }

    fn peek(&mut self) -> Token {

        if self.at_end() {
            return Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
            }
        }

        self.tokens.tokens[self.current].clone()
    }

    fn match_token(&mut self, expected: TokenType) -> bool {
        // Move forward if expected token is present
        if self.at_end() {
            return false;
        }

        if self.tokens.tokens[self.current].token_type != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn at_end(&mut self) -> bool {
        self.current >= self.tokens.tokens.len() || self.advance().token_type == TokenType::Eof
    }
}