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

    LeftParen,
    RightParen,

    LeftAngleBracket,  // <
    RightAngleBracket, // >

    Semicolon,

    Num(i32),
    Ident(String),

    Eof,
}
