use cpast::clex_language::lexer::{Token, TokenType};
use cpast::get_tokens;
#[test]
fn test_single_token() {
    let src = "N";

    assert_eq!(
        get_tokens(src.to_string()).unwrap(),
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
        get_tokens(src.to_string()).unwrap(),
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
        get_tokens(src.to_string()).unwrap(),
        vec![Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
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
            },
            Token {
                token_type: TokenType::LeftSquareBracket,
                lexeme: "[".to_string(),
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
                token_type: TokenType::LiteralNumber(-24),
                lexeme: "-24".to_string(),
            },
            Token {
                token_type: TokenType::RightSquareBracket,
                lexeme: "]".to_string(),
            },
            Token {
                token_type: TokenType::LeftParens,
                lexeme: "(".to_string(),
            },
            Token {
                token_type: TokenType::RightParens,
                lexeme: ")".to_string(),
            },
            Token {
                token_type: TokenType::LeftCurlyBrackets,
                lexeme: "{".to_string(),
            },
            Token {
                token_type: TokenType::RightCurlyBrackets,
                lexeme: "}".to_string(),
            },
            Token {
                token_type: TokenType::Backslash,
                lexeme: "\\".to_string(),
            },
            Token {
                token_type: TokenType::Float,
                lexeme: "F".to_string(),
            },
            Token {
                token_type: TokenType::String,
                lexeme: "S".to_string(),
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
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
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string()
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
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string()
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
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string()
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
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string()
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
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string()
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
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string()
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
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string()
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
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string()
            }
        ]
    );
}

#[test]
fn test_character_invalid() {
    let src = "@CH_NUMBER@";

    assert!(get_tokens(src.to_string()).is_err());
}
