#[derive(Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
}

#[derive(Debug)]
pub enum NodeKind {
    Add,
    Sub,
    Multiply,
    Div,

    // Assign,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,

    Num(i32),
}

impl Node {
    pub fn new_num(n: i32) -> Self {
        Node {
            kind: NodeKind::Num(n),
            lhs: None,
            rhs: None,
        }
    }
}
