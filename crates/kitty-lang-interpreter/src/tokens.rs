use thiserror::{self, Error};

#[derive(Debug, PartialEq)]
pub enum Token {
    // Single character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Period,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Equal,
    Greater,
    Less,

    // Two character tokens.
    BangEqual,
    EqualEqual,
    GreaterEqual,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Float,
    Int,

    // Keywords.
    And,
    Or,
    Not,
    Break,
    Continue,
    If,
    Else,
    True,
    False,
    Null,
    Fn,
    For,
    Var,
    Const,
    Return,
    While,

    // Misc.
    Comment,
    Invalid(InvalidTokenReason),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::Comma => write!(f, ","),
            Token::Period => write!(f, "."),
            Token::Minus => write!(f, "-"),
            Token::Plus => write!(f, "+"),
            Token::Semicolon => write!(f, ";"),
            Token::Slash => write!(f, "/"),
            Token::Star => write!(f, "*"),
            Token::Equal => write!(f, "="),
            Token::Greater => write!(f, ">"),
            Token::Less => write!(f, "<"),
            Token::BangEqual => write!(f, "!="),
            Token::EqualEqual => write!(f, "=="),
            Token::GreaterEqual => write!(f, ">="),
            Token::LessEqual => write!(f, "<="),
            Token::Identifier => write!(f, "<identifier>"),
            Token::String => write!(f, "<string>"),
            Token::Float => write!(f, "<float>"),
            Token::Int => write!(f, "<int>"),
            Token::And => write!(f, "and"),
            Token::Or => write!(f, "or"),
            Token::Not => write!(f, "not"),
            Token::Break => write!(f, "break"),
            Token::Continue => write!(f, "continue"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Null => write!(f, "null"),
            Token::Fn => write!(f, "fn"),
            Token::For => write!(f, "for"),
            Token::Var => write!(f, "var"),
            Token::Const => write!(f, "const"),
            Token::Return => write!(f, "return"),
            Token::While => write!(f, "while"),
            Token::Comment => write!(f, "<comment>"),
            Token::Invalid(invalid_token_reason) => write!(f, "<invalid:{}>", invalid_token_reason),
        }
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum InvalidTokenReason {
    #[error("unrecogonized character(s)")]
    UnknownChars,
    #[error("unterminated string")]
    UnterminatedString,
    #[error("`!` is not supported - use the keyword `not` instead")]
    BangNotSupported,
    #[error("unreogonized characters in number")]
    UnknownNumberChars,
}
