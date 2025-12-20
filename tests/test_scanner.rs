use kitty_lang::{
    scanner::{Lexeme, Scanner},
    tokens::{InvalidTokenReason, Token},
};

#[test]
fn scan_empty_string() {
    assert_eq!(Scanner::new("").collect::<Vec<_>>(), vec![]);
}

#[test]
fn scan_invalid_token() {
    assert_eq!(
        Scanner::new("~").collect::<Vec<_>>(),
        vec![Lexeme {
            token: Token::Invalid(InvalidTokenReason::UnknownChars),
            index: 0,
            length: 1
        }]
    );
}

#[test]
fn scan_single_char_lexemes() {
    assert_eq!(
        Scanner::new("()}{[]-/;,+*").collect::<Vec<_>>(),
        vec![
            Lexeme {
                token: Token::LeftParen,
                index: 0,
                length: 1
            },
            Lexeme {
                token: Token::RightParen,
                index: 1,
                length: 1
            },
            Lexeme {
                token: Token::RightBrace,
                index: 2,
                length: 1
            },
            Lexeme {
                token: Token::LeftBrace,
                index: 3,
                length: 1
            },
            Lexeme {
                token: Token::LeftBracket,
                index: 4,
                length: 1
            },
            Lexeme {
                token: Token::RightBracket,
                index: 5,
                length: 1
            },
            Lexeme {
                token: Token::Minus,
                index: 6,
                length: 1
            },
            Lexeme {
                token: Token::Slash,
                index: 7,
                length: 1
            },
            Lexeme {
                token: Token::Semicolon,
                index: 8,
                length: 1
            },
            Lexeme {
                token: Token::Comma,
                index: 9,
                length: 1
            },
            Lexeme {
                token: Token::Plus,
                index: 10,
                length: 1
            },
            Lexeme {
                token: Token::Star,
                index: 11,
                length: 1
            }
        ]
    );
}

#[test]
fn scanner_disambiguates_two_char_lexemes() {
    assert_eq!(
        Scanner::new("===<<=>>=!=").collect::<Vec<_>>(),
        vec![
            Lexeme {
                token: Token::EqualEqual,
                index: 0,
                length: 2
            },
            Lexeme {
                token: Token::Equal,
                index: 2,
                length: 1
            },
            Lexeme {
                token: Token::Less,
                index: 3,
                length: 1
            },
            Lexeme {
                token: Token::LessEqual,
                index: 4,
                length: 2
            },
            Lexeme {
                token: Token::Greater,
                index: 6,
                length: 1
            },
            Lexeme {
                token: Token::GreaterEqual,
                index: 7,
                length: 2
            },
            Lexeme {
                token: Token::BangEqual,
                index: 9,
                length: 2
            }
        ]
    )
}

#[test]
fn scanner_skips_whitespace() {
    //................01234 567890 123456
    assert_eq!(
        Scanner::new("  == \t= =  \n<>   ").collect::<Vec<_>>(),
        vec![
            Lexeme {
                token: Token::EqualEqual,
                index: 2,
                length: 2
            },
            Lexeme {
                token: Token::Equal,
                index: 6,
                length: 1
            },
            Lexeme {
                token: Token::Equal,
                index: 8,
                length: 1
            },
            Lexeme {
                token: Token::Less,
                index: 12,
                length: 1
            },
            Lexeme {
                token: Token::Greater,
                index: 13,
                length: 1
            },
        ]
    )
}

#[test]
fn scanner_line_comment() {
    //................01234567890
    assert_eq!(
        Scanner::new(" +// hello").collect::<Vec<_>>(),
        vec![
            Lexeme {
                token: Token::Plus,
                index: 1,
                length: 1
            },
            Lexeme {
                token: Token::Comment,
                index: 2,
                length: 8
            },
        ]
    )
}

#[test]
fn scanner_read_strings() {
    //................ 012 3 4567 8901 2345 67890 1
    assert_eq!(
        Scanner::new("\"hi\"\"one\ntwo\"   \"okay\"").collect::<Vec<_>>(),
        vec![
            Lexeme {
                token: Token::String,
                index: 0,
                length: 4
            },
            Lexeme {
                token: Token::String,
                index: 4,
                length: 9
            },
            Lexeme {
                token: Token::String,
                index: 16,
                length: 6
            },
        ]
    )
}

#[test]
fn scanner_error_if_terminating_quote_missing() {
    //................ 012345
    assert_eq!(
        Scanner::new("\"hello").collect::<Vec<_>>(),
        vec![Lexeme {
            token: Token::Invalid(InvalidTokenReason::UnterminatedString),
            index: 0,
            length: 6
        },]
    )
}

#[test]
fn scaner_read_number() {
    //................ 012345
    assert_eq!(
        Scanner::new("\"hello").collect::<Vec<_>>(),
        vec![Lexeme {
            token: Token::Invalid(InvalidTokenReason::UnterminatedString),
            index: 0,
            length: 6
        },]
    )
}

#[test]
fn scanner_read_numbers() {
    //................012345678901234567
    assert_eq!(
        Scanner::new("0.3 -3.1   21   -1").collect::<Vec<_>>(),
        vec![
            Lexeme {
                token: Token::Float,
                index: 0,
                length: 3
            },
            Lexeme {
                token: Token::Float,
                index: 4,
                length: 4
            },
            Lexeme {
                token: Token::Int,
                index: 11,
                length: 2
            },
            Lexeme {
                token: Token::Int,
                index: 16,
                length: 2
            },
        ]
    )
}

#[test]
fn scanner_read_bad_int() {
    //................012345678
    assert_eq!(
        Scanner::new("2p").collect::<Vec<_>>(),
        vec![Lexeme {
            token: Token::Invalid(InvalidTokenReason::UnknownNumberChars),
            index: 0,
            length: 2
        },]
    )
}

#[test]
fn scanner_read_bad_float() {
    //................012345678
    assert_eq!(
        Scanner::new("9.1p").collect::<Vec<_>>(),
        vec![Lexeme {
            token: Token::Invalid(InvalidTokenReason::UnknownNumberChars),
            index: 0,
            length: 4
        },]
    )
}

#[test]
fn scaner_read_identifiers() {
    //................012345678
    assert_eq!(
        Scanner::new("x+yy  zed").collect::<Vec<_>>(),
        vec![
            Lexeme {
                token: Token::Identifier,
                index: 0,
                length: 1
            },
            Lexeme {
                token: Token::Plus,
                index: 1,
                length: 1
            },
            Lexeme {
                token: Token::Identifier,
                index: 2,
                length: 2
            },
            Lexeme {
                token: Token::Identifier,
                index: 6,
                length: 3
            },
        ]
    )
}

#[test]
fn scanner_read_keywords_1() {
    //................0123456789012345678901234
    assert_eq!(
        Scanner::new("and or not break continue").collect::<Vec<_>>(),
        vec![
            Lexeme {
                token: Token::And,
                index: 0,
                length: 3
            },
            Lexeme {
                token: Token::Or,
                index: 4,
                length: 2
            },
            Lexeme {
                token: Token::Not,
                index: 7,
                length: 3
            },
            Lexeme {
                token: Token::Break,
                index: 11,
                length: 5
            },
            Lexeme {
                token: Token::Continue,
                index: 17,
                length: 8
            },
        ]
    )
}

#[test]
fn scanner_read_keywords_2() {
    //................012345678901234567890123456789
    assert_eq!(
        Scanner::new("true false if else null for fn").collect::<Vec<_>>(),
        vec![
            Lexeme {
                token: Token::True,
                index: 0,
                length: 4
            },
            Lexeme {
                token: Token::False,
                index: 5,
                length: 5
            },
            Lexeme {
                token: Token::If,
                index: 11,
                length: 2
            },
            Lexeme {
                token: Token::Else,
                index: 14,
                length: 4
            },
            Lexeme {
                token: Token::Null,
                index: 19,
                length: 4
            },
            Lexeme {
                token: Token::For,
                index: 24,
                length: 3
            },
            Lexeme {
                token: Token::Fn,
                index: 28,
                length: 2
            },
        ]
    )
}

#[test]
fn scanner_read_keywords_3() {
    //................012345678901234567890123456789
    assert_eq!(
        Scanner::new("var const return while").collect::<Vec<_>>(),
        vec![
            Lexeme {
                token: Token::Var,
                index: 0,
                length: 3
            },
            Lexeme {
                token: Token::Const,
                index: 4,
                length: 5
            },
            Lexeme {
                token: Token::Return,
                index: 10,
                length: 6
            },
            Lexeme {
                token: Token::While,
                index: 17,
                length: 5
            },
        ]
    )
}

#[test]
fn scanner_identifers_with_keyword_prefixes() {
    //................012345678901234567890123456789
    assert_eq!(
        Scanner::new("variable contest no").collect::<Vec<_>>(),
        vec![
            Lexeme {
                token: Token::Identifier,
                index: 0,
                length: 8
            },
            Lexeme {
                token: Token::Identifier,
                index: 9,
                length: 7
            },
            Lexeme {
                token: Token::Identifier,
                index: 17,
                length: 2
            },
        ]
    )
}
