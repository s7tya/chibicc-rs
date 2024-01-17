#[derive(Debug)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,

    Equal,              // ==
    NotEqual,           // !=
    LessThanOrEqual,    // <=
    GreaterThanOrEqual, // >=

    LeftParen,
    RightParen,

    LeftAngleBracket,
    RightAngleBracket,

    Num(i32),
}
