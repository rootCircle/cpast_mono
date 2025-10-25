use clex_gen::clex_language::lexer::{Token, TokenType, Span};
use clex_gen::get_tokens;

#[test]
fn test_single_token() {
    let src = "N";

    assert_eq!(
        get_tokens(src.to_string()).unwrap(),
        vec![
            Token {
                token_type: TokenType::Integer,
                lexeme: "N".to_string(),
                span: Span { start: 0, end: 1 },
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                span: Span { start: 1, end: 1 },
            },
        ]
    );
}

#[test]
fn test_empty_source() {
    let src = "";

    assert_eq!(
        get_tokens(src.to_string()).unwrap(),
        vec![Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            span: Span { start: 0, end: 0 },
        }]
    );
}

#[test]
fn test_whitespace_source() {
    let src = "  \t\n\r ";

    assert_eq!(
        get_tokens(src.to_string()).unwrap(),
        vec![Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            span: Span { start: 6, end: 6 },
        }]
    );
}

#[test]
fn test_mixed_tokens() {
    let src = "N [ ?: 42 -24 ] (){}\\ F S";

    assert_eq!(
        get_tokens(src.to_string()).unwrap(),
        vec![
            Token {
                token_type: TokenType::Integer,
                lexeme: "N".to_string(),
                span: Span { start: 0, end: 1 },
            },
            Token {
                token_type: TokenType::LeftSquareBracket,
                lexeme: "[".to_string(),
                span: Span { start: 2, end: 3 },
            },
            Token {
                token_type: TokenType::QuestionColon,
                lexeme: "?:".to_string(),
                span: Span { start: 4, end: 6 },
            },
            Token {
                token_type: TokenType::LiteralNumber(42),
                lexeme: "42".to_string(),
                span: Span { start: 7, end: 9 },
            },
            Token {
                token_type: TokenType::LiteralNumber(-24),
                lexeme: "-24".to_string(),
                span: Span { start: 10, end: 13 },
            },
            Token {
                token_type: TokenType::RightSquareBracket,
                lexeme: "]".to_string(),
                span: Span { start: 14, end: 15 },
            },
            Token {
                token_type: TokenType::LeftParens,
                lexeme: "(".to_string(),
                span: Span { start: 16, end: 17 },
            },
            Token {
                token_type: TokenType::RightParens,
                lexeme: ")".to_string(),
                span: Span { start: 17, end: 18 },
            },
            Token {
                token_type: TokenType::LeftCurlyBrackets,
                lexeme: "{".to_string(),
                span: Span { start: 18, end: 19 },
            },
            Token {
                token_type: TokenType::RightCurlyBrackets,
                lexeme: "}".to_string(),
                span: Span { start: 19, end: 20 },
            },
            Token {
                token_type: TokenType::Backslash,
                lexeme: "\\".to_string(),
                span: Span { start: 20, end: 21 },
            },
            Token {
                token_type: TokenType::Float,
                lexeme: "F".to_string(),
                span: Span { start: 22, end: 23 },
            },
            Token {
                token_type: TokenType::String,
                lexeme: "S".to_string(),
                span: Span { start: 24, end: 25 },
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: String::new(),
                span: Span { start: 25, end: 25 },
            }
        ]
    );
}

#[test]
fn test_characters() {
    let src = "'ABC'";

    assert_eq!(
        get_tokens(src.to_string()).unwrap(),
        vec![
            Token {
                token_type: TokenType::LiteralString("ABC".to_string()),
                lexeme: "ABC".to_string(),
                span: Span { start: 1, end: 4 },
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                span: Span { start: 5, end: 5 },
            }
        ]
    );
}

#[test]
fn test_character_set_upper() {
    let src = "@CH_UPPER@";

    assert_eq!(
        get_tokens(src.to_string()).unwrap(),
        vec![
            Token {
                token_type: TokenType::CharacterSetUpper,
                lexeme: "CH_UPPER".to_string(),
                span: Span { start: 1, end: 9 },
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                span: Span { start: 10, end: 10 },
            }
        ]
    );
}

#[test]
fn test_character_set_lower() {
    let src = "@CH_LOWER@";

    assert_eq!(
        get_tokens(src.to_string()).unwrap(),
        vec![
            Token {
                token_type: TokenType::CharacterSetLower,
                lexeme: "CH_LOWER".to_string(),
                span: Span { start: 1, end: 9 },
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                span: Span { start: 10, end: 10 },
            }
        ]
    );
}

#[test]
fn test_character_set_newline() {
    let src = "@CH_NEWLINE@";

    assert_eq!(
        get_tokens(src.to_string()).unwrap(),
        vec![
            Token {
                token_type: TokenType::CharacterSetNewline,
                lexeme: "CH_NEWLINE".to_string(),
                span: Span { start: 1, end: 11 },
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                span: Span { start: 12, end: 12 },
            }
        ]
    );
}

#[test]
fn test_character_set_alpha() {
    let src = "@CH_ALPHA@";

    assert_eq!(
        get_tokens(src.to_string()).unwrap(),
        vec![
            Token {
                token_type: TokenType::CharacterSetAlpha,
                lexeme: "CH_ALPHA".to_string(),
                span: Span { start: 1, end: 9 },
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                span: Span { start: 10, end: 10 },
            }
        ]
    );
}

#[test]
fn test_character_set_alnum() {
    let src = "@CH_ALNUM@";

    assert_eq!(
        get_tokens(src.to_string()).unwrap(),
        vec![
            Token {
                token_type: TokenType::CharacterSetAlnum,
                lexeme: "CH_ALNUM".to_string(),
                span: Span { start: 1, end: 9 },
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                span: Span { start: 10, end: 10 },
            }
        ]
    );
}

#[test]
fn test_character_set_all() {
    let src = "@CH_ALL@";

    assert_eq!(
        get_tokens(src.to_string()).unwrap(),
        vec![
            Token {
                token_type: TokenType::CharacterSetAll,
                lexeme: "CH_ALL".to_string(),
                span: Span { start: 1, end: 7 },
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                span: Span { start: 8, end: 8 },
            }
        ]
    );
}

#[test]
fn test_character_set_num() {
    let src = "@CH_NUM@";

    assert_eq!(
        get_tokens(src.to_string()).unwrap(),
        vec![
            Token {
                token_type: TokenType::CharacterSetNumeric,
                lexeme: "CH_NUM".to_string(),
                span: Span { start: 1, end: 7 },
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                span: Span { start: 8, end: 8 },
            }
        ]
    );
}

#[test]
fn test_character_invalid() {
    let src = "@CH_NUMBER@";

    assert!(get_tokens(src.to_string()).is_err());
}
