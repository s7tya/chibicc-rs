use crate::tokenizer::{Token, TokenKind};

#[derive(Debug)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Num(i32),
}

#[derive(Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
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

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    cursor: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser { tokens, cursor: 0 }
    }

    pub fn parse(&mut self) -> Node {
        self.expr()
    }

    fn consume(&mut self, op: &str) -> bool {
        if self.cursor >= self.tokens.len() {
            return false;
        }

        let token = &self.tokens[self.cursor];

        let target: String = token.str.chars().take(op.len()).collect();
        if token.kind == TokenKind::Reserved && target == op {
            self.cursor += op.len();
            return true;
        }

        false
    }

    fn expect(&mut self, op: &str) {
        let token = &self.tokens[self.cursor];

        let target: String = token.str.chars().take(op.len()).collect();
        if token.kind == TokenKind::Reserved && target == op {
            self.cursor += 1;
            return;
        }

        panic!(
            "expected {}, found {}.",
            op,
            token.str.chars().next().unwrap()
        );
    }

    fn expect_number(&mut self) -> i32 {
        let token = &self.tokens[self.cursor];

        if matches!(token.kind, TokenKind::Num(_)) {
            self.cursor += 1;

            if let TokenKind::Num(n) = token.kind {
                return n;
            }
        }

        panic!("expected number, found {:#?}.", token.kind)
    }

    fn expr(&mut self) -> Node {
        let mut node = self.equality();

        if self.consume("+") {
            node = Node {
                kind: NodeKind::Add,
                lhs: Some(Box::from(node)),
                rhs: Some(Box::from(self.equality())),
            };
        } else if self.consume("-") {
            node = Node {
                kind: NodeKind::Sub,
                lhs: Some(Box::from(node)),
                rhs: Some(Box::from(self.equality())),
            }
        }

        node
    }

    fn equality(&mut self) -> Node {
        let mut node = self.relational();

        if self.consume("==") {
            node = Node {
                kind: NodeKind::Eq,
                lhs: Some(Box::from(node)),
                rhs: Some(Box::from(self.relational())),
            };
        } else if self.consume("!=") {
            node = Node {
                kind: NodeKind::Ne,
                lhs: Some(Box::from(node)),
                rhs: Some(Box::from(self.relational())),
            };
        }

        node
    }

    fn relational(&mut self) -> Node {
        let mut node = self.add();

        if self.consume("<") {
            node = Node {
                kind: NodeKind::Lt,
                lhs: Some(Box::from(node)),
                rhs: Some(Box::from(self.add())),
            };
        } else if self.consume("<=") {
            node = Node {
                kind: NodeKind::Le,
                lhs: Some(Box::from(node)),
                rhs: Some(Box::from(self.add())),
            }
        } else if self.consume(">") {
            node = Node {
                kind: NodeKind::Lt,
                lhs: Some(Box::from(self.add())),
                rhs: Some(Box::from(node)),
            };
        } else if self.consume(">=") {
            node = Node {
                kind: NodeKind::Le,
                lhs: Some(Box::from(self.add())),
                rhs: Some(Box::from(node)),
            }
        }

        node
    }

    fn add(&mut self) -> Node {
        let mut node = self.mul();

        if self.consume("+") {
            node = Node {
                kind: NodeKind::Add,
                lhs: Some(Box::from(node)),
                rhs: Some(Box::from(self.mul())),
            };
        } else if self.consume("-") {
            node = Node {
                kind: NodeKind::Sub,
                lhs: Some(Box::from(node)),
                rhs: Some(Box::from(self.mul())),
            }
        }

        node
    }

    fn mul(&mut self) -> Node {
        let mut node = self.unary();

        if self.consume("*") {
            node = Node {
                kind: NodeKind::Mul,
                lhs: Some(Box::from(node)),
                rhs: Some(Box::from(self.unary())),
            };
        } else if self.consume("/") {
            node = Node {
                kind: NodeKind::Div,
                lhs: Some(Box::from(node)),
                rhs: Some(Box::from(self.unary())),
            }
        }

        node
    }

    fn unary(&mut self) -> Node {
        if self.consume("+") {
            return self.primary();
        }

        if self.consume("-") {
            return Node {
                kind: NodeKind::Sub,
                lhs: Some(Box::new(Node::new_num(0))),
                rhs: Some(Box::new(self.primary())),
            };
        }

        self.primary()
    }

    fn primary(&mut self) -> Node {
        if self.consume("(") {
            let node = self.expr();
            self.expect(")");

            return node;
        }

        return Node {
            kind: NodeKind::Num(self.expect_number()),
            lhs: None,
            rhs: None,
        };
    }
}
