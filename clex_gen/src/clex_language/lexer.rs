//! The `lexer` module provides lexical analysis capabilities for tokenizing input patterns in the `clex` language.
//! It defines the `Tokens` struct, which represents a collection of tokens produced by the lexer.
//! The lexer scans the input pattern and identifies different types of tokens, categorizing them according to the `TokenType` enum.
//!
//! # Types
//!
//! - `Tokens`: Represents a collection of tokens produced by the lexer, containing information about each token's type and lexeme.
//! - `Token`: Represents an individual token, consisting of a `TokenType` and the corresponding lexeme.
//! - `TokenType`: Enumerates different types of tokens that the lexer can identify, including metacharacters, character sets, literals, and end-of-file markers.
//!
//! The lexer is a crucial component in the `clex_language` module, providing the initial step in processing the `clex` language input patterns.
//! It tokenizes the input, making it easier for subsequent components, such as the parser, to understand and structure the code patterns.
//!
//! # Example
//!
//! ```rust
//! use clex_gen::clex_language::lexer::{TokenType, Token, Span};
//! use clex_gen::get_tokens;
//!
//! // Example input pattern
//! let src = "N";
//!
//! // Assert token types and lexemes in a test
//! assert_eq!(
//!     get_tokens(src.to_string()).unwrap(),
//!     vec![
//!         Token {
//!             token_type: TokenType::Integer,
//!             lexeme: "N".to_string(),
//!             span: Span { start: 0, end: 1 },
//!         },
//!         Token {
//!             token_type: TokenType::Eof,
//!             lexeme: "".to_string(),
//!             span: Span { start: 1, end: 1 },
//!         },
//!     ]
//! );
//! ```
//!
//! For more details on the types and methods provided by the lexer, refer to the documentation for each type.

use crate::clex_language::clex_error_type::{ClexErrorType, ParentErrorType};
use unicode_segmentation::UnicodeSegmentation;

/// Represents a span (position range) in the source code
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Span {
    /// Starting position (byte offset) in the source
    pub start: usize,
    /// Ending position (byte offset) in the source
    pub end: usize,
}

/// Represents the different types of tokens in the lexer.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
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
    /// Integer token type
    Integer,
    /// Float token type
    Float,
    /// String token type.
    String,

    // Character Sets
    CharacterSetAlpha,
    CharacterSetAlnum,
    CharacterSetNewline,
    CharacterSetNumeric,
    CharacterSetUpper,
    CharacterSetLower,
    CharacterSetAll,

    // Literals
    /// Literal number token type with a specified value.
    LiteralNumber(i64),
    LiteralString(String),

    // End of file
    Eof,
}

/// Represents a token in the lexer, consisting of a token type and the corresponding lexeme.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    /// The type of the token, indicating its classification.
    pub token_type: TokenType,
    /// The actual characters that form the token in the source code.
    pub lexeme: String,
    /// The span (position range) of this token in the source code
    pub span: Span,
}

/// Represents a collection of tokens produced by the lexer.
#[derive(Debug, Clone)]
pub struct Tokens {
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    source_language: String,
}

impl Tokens {
    /// Creates a new Tokens instance for a given source language.
    pub fn new(source_language: String) -> Self {
        Self {
            tokens: Vec::new(),
            start: 0,
            current: 0,
            source_language,
        }
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    /// Scans tokens from the source language and adds them to the tokens vector.
    pub fn scan_tokens(&mut self) -> Result<(), ClexErrorType> {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            span: Span {
                start: self.current,
                end: self.current,
            },
        });

        Ok(())
    }

    /// Checks if the lexer has reached the end of the source language.
    fn at_end(&self) -> bool {
        self.source_language.len() <= self.current
    }

    /// Scans a single token from the source language.
    fn scan_token(&mut self) -> Result<(), ClexErrorType> {
        let c = self.advance().to_string(); // Convert to owned String to avoid borrow issues
        match c.as_str() {
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
            "@" => {
                let start_pos = self.start; // Store the starting position
                self.start += 1; // Skip first character in lexeme

                let mut literal = String::new();
                while self.peek() != "@" && !self.at_end() {
                    literal.push_str(self.advance());
                }

                // Unterminated string.
                if self.at_end() {
                    return Err(ClexErrorType::UnclosedAtSymbol(
                        ParentErrorType::LexerError,
                        Span {
                            start: start_pos,
                            end: self.current,
                        },
                    ));
                }

                let token_type = match literal.trim().to_uppercase().as_str() {
                    "CH_ALPHA" => TokenType::CharacterSetAlpha,
                    "CH_NUM" => TokenType::CharacterSetNumeric,
                    "CH_NEWLINE" => TokenType::CharacterSetNewline,
                    "CH_ALNUM" => TokenType::CharacterSetAlnum,
                    "CH_UPPER" => TokenType::CharacterSetUpper,
                    "CH_LOWER" => TokenType::CharacterSetLower,
                    "CH_ALL" => TokenType::CharacterSetAll,
                    _ => {
                        return Err(ClexErrorType::InvalidCharacterSet(
                            ParentErrorType::LexerError,
                            Span {
                                start: start_pos,
                                end: self.current,
                            },
                        ));
                    }
                };

                self.add_token(token_type);
                if !self.match_str("@") {
                    return Err(ClexErrorType::UnclosedAtSymbol(
                        ParentErrorType::LexerError,
                        Span {
                            start: start_pos,
                            end: self.current,
                        },
                    ));
                }
            }
            " " | "\r" | "\t" | "\n" => {
                // Do nothing, just ignore these spaces
            }
            "'" => {
                let start_pos = self.start; // Store the starting position
                self.start += 1; // Skip first character in lexeme

                let mut literal = String::new();
                while self.peek() != "'" && !self.at_end() {
                    let c = self.advance();
                    if c == "\\" {
                        let escaped = match self.peek() {
                            "n" => '\n',   // newline
                            "t" => '\t',   // tab
                            "r" => '\r',   // carriage return
                            "\\" => '\\',  // backslash
                            "'" => '\'',   // single quote
                            "\"" => '\"',  // double quote
                            "0" => '\0',   // null
                            "a" => '\x07', // bell
                            "b" => '\x08', // backspace
                            "f" => '\x0C', // form feed
                            "v" => '\x0B', // vertical tab
                            _ => '\\',
                        };
                        self.advance();
                        literal.push(escaped);
                    } else {
                        literal.push_str(c);
                    }
                }

                if self.at_end() {
                    return Err(ClexErrorType::UnclosedSingleQuotes(
                        ParentErrorType::LexerError,
                        Span {
                            start: start_pos,
                            end: self.current,
                        },
                    ));
                }

                self.add_token(TokenType::LiteralString(literal));
                if !self.match_str("'") {
                    return Err(ClexErrorType::UnclosedSingleQuotes(
                        ParentErrorType::LexerError,
                        Span {
                            start: start_pos,
                            end: self.current,
                        },
                    ));
                }
            }
            "?" => {
                let start_pos = self.start;
                if self.match_str(":") {
                    self.add_token(TokenType::QuestionColon);
                } else {
                    return Err(ClexErrorType::MissingColonAfterQuestionMark(
                        ParentErrorType::LexerError,
                        Span {
                            start: start_pos,
                            end: self.current,
                        },
                    ));
                }
            }
            _ => {
                if c.as_str() == "-" || Self::is_digit(c.as_str()) {
                    let start_pos = self.start;
                    if c.as_str() == "-" && !Self::is_digit(self.peek()) {
                        return Err(ClexErrorType::MissingNumberAfterNegativeSign(
                            ParentErrorType::LexerError,
                            Span {
                                start: start_pos,
                                end: self.current,
                            },
                        ));
                    }

                    while Self::is_digit(self.peek()) {
                        self.current += 1;
                    }

                    let number = match self.source_language[self.start..self.current].parse::<i64>()
                    {
                        Ok(num) => num,
                        Err(_err) => {
                            return Err(ClexErrorType::NumericParsingError(
                                ParentErrorType::LexerError,
                                Span {
                                    start: start_pos,
                                    end: self.current,
                                },
                            ));
                        }
                    };

                    self.add_token(TokenType::LiteralNumber(number));
                } else {
                    let character: &'static str = Box::leak(c.into());
                    return Err(ClexErrorType::UnknownCharacter(
                        ParentErrorType::LexerError,
                        Span {
                            start: self.start,
                            end: self.current,
                        },
                        character,
                    ));
                }
            }
        }
        Ok(())
    }

    /// Adds a token to the tokens vector.
    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            lexeme: self.source_language[self.start..self.current].to_string(),
            span: Span {
                start: self.start,
                end: self.current,
            },
        });
    }

    /// Advances the current index and returns the character at the new index.
    fn advance(&mut self) -> &str {
        self.current += 1;
        self.char_at(self.current - 1)
    }

    /// Retrieves the character at the specified index.
    fn char_at(&self, index: usize) -> &str {
        self.source_language.graphemes(true).collect::<Vec<&str>>()[index]
    }

    /// Checks if the current characters match the expected string if yes then traverse as well.
    fn match_str(&mut self, expected: &str) -> bool {
        if self.at_end() || self.char_at(self.current) != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    /// Checks if the character is a digit.
    fn is_digit(ch: &str) -> bool {
        ("0"..="9").contains(&ch)
    }

    /// Peeks at the character at the current index.
    fn peek(&self) -> &str {
        if self.at_end() {
            "\0"
        } else {
            self.char_at(self.current)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenization_works() {
        let src = "12N3";
        let mut tokens = Tokens::new(src.to_string());

        tokens.scan_tokens().unwrap();

        assert_eq!(
            tokens.tokens,
            vec![
                Token {
                    token_type: TokenType::LiteralNumber(12),
                    lexeme: "12".to_string(),
                    span: Span { start: 0, end: 2 },
                },
                Token {
                    token_type: TokenType::Integer,
                    lexeme: "N".to_string(),
                    span: Span { start: 2, end: 3 },
                },
                Token {
                    token_type: TokenType::LiteralNumber(3),
                    lexeme: "3".to_string(),
                    span: Span { start: 3, end: 4 },
                },
                Token {
                    token_type: TokenType::Eof,
                    lexeme: String::new(),
                    span: Span { start: 4, end: 4 },
                }
            ]
        );
    }
}
