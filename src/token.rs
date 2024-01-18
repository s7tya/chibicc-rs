#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,

    Assign, // =

    Equal,              // ==
    NotEqual,           // !=
    LessThanOrEqual,    // <=
    GreaterThanOrEqual, // >=

    LeftParen,  // (
    RightParen, // )

    LeftBrace,  // {
    RightBrace, // }

    LeftAngleBracket,  // <
    RightAngleBracket, // >

    Semicolon,

    Num(i32),
    Ident(String),

    Return,
    If,
    Else,
    While,
    For,

    Eof,
}
