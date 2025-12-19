///
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
    Identifier {
        /// Index of the identifier's first character.
        start_index: usize,
        /// Number of characters in the literal.
        len: usize,
    },
    String {
        /// Index of the string's first character past the initial quote. This is `None` when the
        /// string is empty (length 0).
        start: Option<usize>,
        /// Number of characters in the string.
        len: usize,
    },
    Float(f64),
    Int(i64),

    // Keywords.
    And,
    Or,
    Break,
    Continue,
    Else,
    False,
    Fn,
    For,
    If,
    Var,
    Const,
    Null,
    Return,
    True,
    While,

    // Misc.
    Comment,
    Invalid,
}
