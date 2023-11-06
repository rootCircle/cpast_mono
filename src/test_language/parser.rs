use crate::test_language::ast::{AstLanguage, DataType, Expression, UnitExpression};
use crate::test_language::lexer::Token;
use super::lexer::{Tokens, TokenType};

struct Parser {
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
            self.language.tokens.push(self.parse_expr());
        }

        AstLanguage { tokens: vec![] }
    }

    fn peek(&mut self) -> Token {
        self.current += 1;
        self.tokens.tokens[self.current].clone()
    }

    fn match_token(&mut self, expected: TokenType) -> bool {
        if self.at_end() {
            return false;
        }

        if self.tokens.tokens[self.current].token_type != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn parse_expr(&self) -> Expression {
        Expression { unit: vec![] }
    }

    fn view(&mut self, expected: TokenType) -> i64 {
        if let Some(index) = self.tokens.tokens.iter().skip(self.current).position(|item| item.token_type == expected) {
            return (index + self.current) as i64;
        }
        -1
    }

    fn parse_capturing_group(&mut self) -> Option<UnitExpression> {
        let tk = self.peek();

        if tk.token_type == TokenType::LeftParens && self.match_token(TokenType::Integer) && self.match_token(TokenType::RightParens) {
            self.groups += 1;
            return Some(UnitExpression::CapturingGroup {
                group_number: self.groups,
            });
        }
        None
    }

    fn parse_non_capturing_group(&mut self) -> Option<UnitExpression> {
        let tk = self.peek();

        if tk.token_type == TokenType::LeftParens && self.match_token(TokenType::ColonQuestion) {
            // let sub_token = Parser::new(
            //
            // );
            return Some(UnitExpression::NonCapturingGroup {
                nest_exp: Expression { unit: vec![] },
            });

        }
        None
    }

    fn parse_primary(&mut self) -> Option<UnitExpression> {
        let tk = self.peek();

        match tk.token_type {
            TokenType::Integer => Some(UnitExpression::Primitives {data_type: DataType::Integer}),
            TokenType::Float => Some(UnitExpression::Primitives {data_type: DataType::Float}),
            TokenType::String => Some(UnitExpression::Primitives {data_type: DataType::String}),
            TokenType::Character => Some(UnitExpression::Primitives {data_type: DataType::Character}),
            _ => None
        }
    }

    fn at_end(&mut self) -> bool {
        self.current >= self.tokens.tokens.len() || self.peek().token_type == TokenType::Eof
    }
}