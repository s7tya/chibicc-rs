use crate::tokenizer::{Token, TokenKind};

#[derive(Debug)]
enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Num(i32),
}

#[derive(Debug)]
pub struct Node {
    kind: NodeKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    cursor: usize,
}
//  && token.input[0] == op

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser { tokens, cursor: 0 }
    }

    fn consume(&mut self, op: char) -> bool {
        if self.cursor >= self.tokens.len() {
            return false;
        }

        let token = &self.tokens[self.cursor];

        if token.kind == TokenKind::Reserved && token.str.chars().nth(0).unwrap() == op {
            self.cursor += 1;
            return true;
        }

        false
    }

    fn expect(&mut self, op: char) {
        let token = &self.tokens[self.cursor];

        if token.kind == TokenKind::Reserved && token.str.chars().next().unwrap() == op {
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

    pub fn expr(&mut self) -> Node {
        let mut node = self.mul();

        loop {
            if self.consume('+') {
                node = Node {
                    kind: NodeKind::Add,
                    lhs: Some(Box::from(node)),
                    rhs: Some(Box::from(self.mul())),
                };
            } else if self.consume('-') {
                node = Node {
                    kind: NodeKind::Sub,
                    lhs: Some(Box::from(node)),
                    rhs: Some(Box::from(self.mul())),
                }
            } else {
                return node;
            }
        }
    }
    fn mul(&mut self) -> Node {
        let mut node = self.primary();

        loop {
            if self.consume('*') {
                node = Node {
                    kind: NodeKind::Mul,
                    lhs: Some(Box::from(node)),
                    rhs: Some(Box::from(self.primary())),
                };
            } else if self.consume('/') {
                node = Node {
                    kind: NodeKind::Div,
                    lhs: Some(Box::from(node)),
                    rhs: Some(Box::from(self.primary())),
                }
            } else {
                return node;
            }
        }
    }

    fn primary(&mut self) -> Node {
        if self.consume('(') {
            let node = self.expr();
            self.expect(')');

            return node;
        }

        return Node {
            kind: NodeKind::Num(self.expect_number()),
            lhs: None,
            rhs: None,
        };
    }
}
