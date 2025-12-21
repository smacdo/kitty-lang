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
    Bang,

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

#[derive(Debug, PartialEq)]
pub enum InvalidTokenReason {
    UnknownChars,
    UnterminatedString,
    BangNotSupported,
    UnknownNumberChars,
}
