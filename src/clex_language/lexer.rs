use std::process::exit;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TokenType {
    // Metacharacters
    LeftParens,
    RightParens,
    QuestionColon,
    Backslash,
    LeftSquareBracket,
    RightSquareBracket,
    LeftCurlyBrackets,
    RightCurlyBrackets,
    Comma,

    // Character sets
    Integer, // (Value, Min, Max)
    Float,   // (Value, Min, Max)
    String,
    Character,
    // Space,

    // Literals
    LiteralNumber(i64),

    // End of file
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
}

pub(crate) struct Tokens {
    pub(crate) tokens: Vec<Token>,
    start: usize,
    current: usize,
    source_language: String,
}

impl Tokens {
    pub fn new(source_language: String) -> Self {
        Self {
            tokens: Vec::new(),
            start: 0,
            current: 0,
            source_language,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.at_end() {
            self.start = self.current;
            let scan_token = self.scan_token();
            if let Err(err) = scan_token {
                eprintln!("[LEXER ERROR] {}", err);
                exit(1);
            }
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
        });
    }

    fn at_end(&self) -> bool {
        self.source_language.len() <= self.current
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();
        match c {
            "(" => self.add_token(TokenType::LeftParens),
            ")" => self.add_token(TokenType::RightParens),
            "[" => self.add_token(TokenType::LeftSquareBracket),
            "]" => self.add_token(TokenType::RightSquareBracket),
            "{" => self.add_token(TokenType::LeftCurlyBrackets),
            "}" => self.add_token(TokenType::RightCurlyBrackets),
            "," => self.add_token(TokenType::Comma),
            "\\" => self.add_token(TokenType::Backslash),
            "N" => self.add_token(TokenType::Integer),
            "F" => self.add_token(TokenType::Float),
            "S" => self.add_token(TokenType::String),
            "C" => self.add_token(TokenType::Character),
            " " | "\r" | "\t" | "\n" => {
                // Do nothing, just those spaces out :evil:
            }
            "?" => {
                if self.match_str(":") {
                    self.add_token(TokenType::QuestionColon);
                } else {
                    return Err("Expected : after ?".to_string());
                }
            }
            _ => {
                if c == "-" || Tokens::is_digit(c) {
                    if c == "-" && !Tokens::is_digit(self.peek()) {
                        return Err("Expected Number after -".to_string());
                    }

                    while Tokens::is_digit(self.peek()) {
                        self.current += 1;
                    }

                    let number = match self.source_language[self.start..self.current].parse::<i64>()
                    {
                        Ok(num) => num,
                        Err(_err) => {
                            return Err("Error parsing the number".to_string());
                        }
                    };

                    self.add_token(TokenType::LiteralNumber(number));
                } else {
                    return Err(format!("Unexpected character {c}"));
                }
            }
        }
        Ok(())
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            lexeme: self.source_language[self.start..self.current].to_string(),
        })
    }

    fn advance(&mut self) -> &str {
        self.current += 1;
        self.char_at(self.current - 1)
    }

    fn char_at(&self, index: usize) -> &str {
        self.source_language.graphemes(true).collect::<Vec<&str>>()[index]
    }

    fn match_str(&mut self, expected: &str) -> bool {
        if self.at_end() {
            return false;
        }

        if self.char_at(self.current) != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn is_digit(ch: &str) -> bool {
        ("0"..="9").contains(&ch)
    }

    fn peek(&self) -> &str {
        if self.at_end() {
            return "\0";
        }

        self.char_at(self.current)
    }
}

#[cfg(test)]
mod tests {
    use crate::clex_language::lexer::{Token, TokenType, Tokens};

    #[test]
    fn tokenization_works() {
        let mut src = "12N3";
        let mut tokens = Tokens::new(src.to_string());

        tokens.scan_tokens();

        assert_eq!(
            tokens.tokens,
            vec![
                Token {
                    token_type: TokenType::LiteralNumber(12),
                    lexeme: "12".to_string()
                },
                Token {
                    token_type: TokenType::Integer,
                    lexeme: "N".to_string()
                },
                Token {
                    token_type: TokenType::LiteralNumber(3),
                    lexeme: "3".to_string()
                },
                Token {
                    token_type: TokenType::Eof,
                    lexeme: "".to_string()
                }
            ]
        );
    }
}
