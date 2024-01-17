use crate::token::Token;

pub enum NodeKind {
    Add,
    Sub,
    Div,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    Num(i32),
}

pub struct Node {
    kind: NodeKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cursor: 0 }
    }

    pub fn parse(&mut self) {}
}
