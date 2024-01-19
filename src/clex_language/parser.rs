use super::lexer::{TokenType, Tokens};
use crate::clex_language::ast::{
    CharacterSet, DataType, Program, ReferenceType, UnitExpression, MAX_STRING_SIZE,
};
use crate::clex_language::lexer::Token;
use std::process::exit;

#[derive(Debug)]
pub(crate) struct Parser {
    tokens: Tokens,
    start: usize,
    current: usize,
    pub(crate) language: Program,
    current_group: u64, // for capturing groupCount, starts from 1.....
}

impl Parser {
    pub fn new(source_language: String) -> Self {
        let mut tokens = Tokens::new(source_language);
        tokens.scan_tokens();

        Self {
            tokens,
            start: 0,
            current: 0,
            language: Program { expression: vec![] },
            current_group: 0,
        }
    }

    pub fn new_from_tokens(tokens: Tokens) -> Self {
        Self {
            tokens,
            start: 0,
            current: 0,
            language: Program { expression: vec![] },
            current_group: 0,
        }
    }

    pub fn parser(&mut self) {
        while !self.at_end() {
            self.start = self.current;

            let expr = self.parse_expr();
            self.language.expression.push(expr);
        }
    }

    pub fn parse_expr(&mut self) -> UnitExpression {
        let token = self.advance();

        match token.token_type {
            TokenType::Integer => {
                let (lower_bound, upper_bound) = self.parse_range();
                let repetition_type = self.parse_quantifier();

                UnitExpression::Primitives {
                    data_type: DataType::Integer(lower_bound, upper_bound),
                    repetition: repetition_type,
                }
            }
            TokenType::Float => {
                let (lower_reference, upper_reference) = self.parse_range();
                let repetition_type = self.parse_quantifier();

                UnitExpression::Primitives {
                    data_type: DataType::Float(lower_reference, upper_reference),
                    repetition: repetition_type,
                }
            }
            TokenType::String => {
                let (length, charset) = self.parse_string_modifiers();
                let repetition_type = self.parse_quantifier();

                UnitExpression::Primitives {
                    data_type: DataType::String(length, charset),
                    repetition: repetition_type,
                }
            }
            TokenType::Character => {
                let repetition_type = self.parse_quantifier();

                UnitExpression::Primitives {
                    data_type: DataType::Character,
                    repetition: repetition_type,
                }
            }
            TokenType::LeftParens => {
                if self.match_token(TokenType::Integer) {
                    let (mut lower_reference, upper_reference) = self.parse_range();
                    if !self.match_token(TokenType::RightParens) {
                        eprintln!("[PARSER ERROR] Expected ) after (N in Capturing Group");
                        exit(1);
                    }

                    if let ReferenceType::ByLiteral(value) = lower_reference {
                        if value < 0 {
                            lower_reference = ReferenceType::ByLiteral(0);
                        }
                    }

                    self.current_group += 1;

                    UnitExpression::CapturingGroup {
                        group_number: self.current_group,
                        data_type: DataType::Integer(lower_reference, upper_reference),
                    }
                } else if self.match_token(TokenType::QuestionColon) {
                    let last_index =
                        self.peek_from_current(TokenType::RightParens, TokenType::LeftParens);

                    let last_index = match last_index {
                        Some(t) => t,
                        None => {
                            eprintln!("[PARSER ERROR] Can't find the closing Right Parens in Non Capturing Group");
                            exit(1);
                        }
                    };

                    let mut nest_exp = Vec::new();

                    while self.current < last_index {
                        let expr = self.parse_expr();
                        match expr {
                            UnitExpression::Primitives { .. }
                            | UnitExpression::NonCapturingGroup { .. }
                            | UnitExpression::CapturingGroup { .. } => {
                                nest_exp.push(expr);
                            }
                            // UnitExpression::CapturingGroup { .. } => {
                            //     eprintln!("[PARSER ERROR] Capturing Group isn't allowed inside Non-capturing Group");
                            //     eprintln!("[PARSER ERROR] Prefer removing the non-capturing group if there is no quantifier");
                            //     exit(1);
                            // },
                            // Unreachable Code imo
                            UnitExpression::Eof => break,
                        }
                    }

                    if !self.match_token(TokenType::RightParens) {
                        eprintln!("[PARSER ERROR] Expected a closing parens ) after (:? in non-capturing group");
                        exit(1);
                    }

                    // Move till N
                    let repetition_type = self.parse_quantifier();
                    UnitExpression::NonCapturingGroup {
                        nest_exp,
                        repetition: repetition_type,
                    }
                } else {
                    eprintln!("[PARSER ERROR] Expected N) or :?<ChildExpression> after opening (");
                    exit(1);
                }
            }
            TokenType::Eof => UnitExpression::Eof,
            _ => {
                eprintln!("[PARSER ERROR] Invalid Token found!");
                eprintln!("[PARSER ERROR] {:#?}", token.token_type);
                exit(1);
            }
        }
    }

    fn parse_quantifier(&mut self) -> ReferenceType {
        if self.match_token(TokenType::LeftCurlyBrackets) {
            let reference = self.parse_reference();

            if let ReferenceType::ByLiteral(count) = reference {
                if count <= 0 {
                    eprintln!("[PARSER ERROR] Count in Quantifier can't be 0 or negative!");
                    exit(1);
                }
            }

            if reference == ReferenceType::None {
                eprintln!("[PARSER ERROR] Expected \\N}} or N}} after {{");
                exit(1);
            }

            if !self.match_token(TokenType::RightCurlyBrackets) {
                eprintln!("[PARSER ERROR] Expected }} after {{\\N in Quantifiers");
                exit(1);
            }
            return reference;
        }
        ReferenceType::None
    }

    fn parse_string_modifiers(&mut self) -> (ReferenceType, CharacterSet) {
        let length = MAX_STRING_SIZE;
        let mut length_reference = ReferenceType::ByLiteral(length as i64);
        let mut char_set = CharacterSet::default_charset();

        if self.match_token(TokenType::LeftSquareBracket) {
            length_reference = self.parse_reference();

            if length_reference == ReferenceType::None {
                length_reference = ReferenceType::ByLiteral(length as i64);
            }

            if !self.match_token(TokenType::Comma) {
                eprintln!("[PARSER ERROR] Expected , after [ in String Modifier Expression");
                exit(1);
            }

            if let TokenType::LiteralCharacter(char) = self.tokens.tokens[self.current].token_type {
                char_set = CharacterSet::get_charset_from_code(char);
                self.advance();
            }

            if !self.match_token(TokenType::RightSquareBracket) {
                eprintln!(
                    "[PARSER ERROR] Expected character or ] after [ in String Modifier Expression"
                );
                exit(1);
            }
        }

        (length_reference, char_set)
    }
    fn parse_range(&mut self) -> (ReferenceType, ReferenceType) {
        let lower_bound = i64::MIN;
        let upper_bound = i64::MAX;
        let mut lower_reference = ReferenceType::ByLiteral(lower_bound);
        let mut upper_reference = ReferenceType::ByLiteral(upper_bound);

        if self.match_token(TokenType::LeftSquareBracket) {
            lower_reference = self.parse_reference();

            if lower_reference == ReferenceType::None {
                lower_reference = ReferenceType::ByLiteral(lower_bound);
            }

            if !self.match_token(TokenType::Comma) {
                eprintln!("[PARSER ERROR] Expected , after [ in Range Bound Expression");
                exit(1);
            }

            upper_reference = self.parse_reference();

            if upper_reference == ReferenceType::None {
                upper_reference = ReferenceType::ByLiteral(upper_bound);
            }

            if !self.match_token(TokenType::RightSquareBracket) {
                eprintln!("[PARSER ERROR] Expected ] after [ in Range Bound Expression");
                exit(1);
            }
        }

        (lower_reference, upper_reference)
    }

    fn parse_reference(&mut self) -> ReferenceType {
        if self.match_token(TokenType::Backslash) {
            if let TokenType::LiteralNumber(value) = self.peek().token_type {
                self.current += 1;

                if value <= 0 {
                    eprintln!(
                        "[PARSER ERROR] Group Number in Back-reference can't be 0 or negative!"
                    );
                    exit(1);
                }

                ReferenceType::ByGroup {
                    group_number: value as u64,
                }
            } else {
                eprintln!("[PARSER ERROR] Expected <Group Number> after {{\\ in Quantifiers");
                exit(1);
            }
        } else if let TokenType::LiteralNumber(value) = self.peek().token_type {
            self.current += 1;

            return ReferenceType::ByLiteral(value);
        } else {
            return ReferenceType::None;
        }
    }

    fn peek_from_current(&mut self, expected: TokenType, not_expected: TokenType) -> Option<usize> {
        // Finds index of occurrence of expected Token from current position
        // while self.lookup(expected, self.current)
        let mut stack = Vec::new();
        let current_reset_duplicate = self.current;

        while !self.at_end() {
            let tk = self.advance();

            if tk.token_type == not_expected {
                stack.push(&not_expected)
            } else if tk.token_type == expected {
                if let Some(not_expect) = stack.pop() {
                    if not_expect == &not_expected {
                        stack.pop();
                    } else {
                        // Unreachable Code imo XD
                        stack.push(&expected);
                    }
                } else {
                    let expected_index = self.current - 1;
                    self.current = current_reset_duplicate;
                    return Some(expected_index);
                }
            }
        }

        self.current = current_reset_duplicate;

        None
    }

    fn lookup(&mut self, expected: TokenType, current: usize) -> i64 {
        // Finds index of occurrence of expected Token from current position
        if let Some(index) = self
            .tokens
            .tokens
            .iter()
            .skip(current)
            .position(|item| item.token_type == expected)
        {
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
            };
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
        self.current >= self.tokens.tokens.len()
    }
}
