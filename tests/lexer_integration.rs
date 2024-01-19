use cpast::clex_language::lexer::{Token, TokenType};
use cpast::get_tokens;
#[test]
fn test_single_token() {
    let src = "N";

    assert_eq!(
        get_tokens(src.to_string()),
        vec![
            Token {
                token_type: TokenType::Integer,
                lexeme: "N".to_string(),
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
            },
        ]
    );
}

#[test]
fn test_empty_source() {
    let src = "";

    assert_eq!(
        get_tokens(src.to_string()),
        vec![Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
        }]
    );
}

#[test]
fn test_whitespace_source() {
    let src = "  \t\n\r ";

    assert_eq!(
        get_tokens(src.to_string()),
        vec![Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
        }]
    );
}

#[test]
fn test_mixed_tokens() {
    let src = "N [ C ?: 42 ]";

    assert_eq!(
        get_tokens(src.to_string()),
        vec![
            Token {
                token_type: TokenType::Integer,
                lexeme: "N".to_string(),
            },
            Token {
                token_type: TokenType::LeftSquareBracket,
                lexeme: "[".to_string(),
            },
            Token {
                token_type: TokenType::Character,
                lexeme: "C".to_string(),
            },
            Token {
                token_type: TokenType::QuestionColon,
                lexeme: "?:".to_string(),
            },
            Token {
                token_type: TokenType::LiteralNumber(42),
                lexeme: "42".to_string(),
            },
            Token {
                token_type: TokenType::RightSquareBracket,
                lexeme: "]".to_string(),
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
            },
        ]
    );
}

#[test]
fn test_characters() {
    let src = "'A'";

    assert_eq!(
        get_tokens(src.to_string()),
        vec![
            Token {
                token_type: TokenType::LiteralCharacter('A'),
                lexeme: "'A".to_string(),
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string()
            }
        ]
    );
}
