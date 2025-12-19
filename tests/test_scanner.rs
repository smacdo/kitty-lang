use kitty_lang::{
    scanner::{Lexeme, Scanner},
    tokens::Token,
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
            token: Token::Invalid,
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
        Scanner::new("===<<=>>=!!=").collect::<Vec<_>>(),
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
                token: Token::Bang,
                index: 9,
                length: 1
            },
            Lexeme {
                token: Token::BangEqual,
                index: 10,
                length: 2
            },
        ]
    )
}
