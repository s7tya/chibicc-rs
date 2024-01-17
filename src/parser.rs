use crate::{
    node::{Node, NodeKind},
    token::Token,
};

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cursor: 0 }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        self.program()
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.cursor).cloned()
    }

    fn consume(&mut self, token: Token) -> bool {
        let next = self.peek().unwrap();
        if next == token {
            self.cursor += 1;
            return true;
        }

        false
    }

    fn expect(&mut self, token: Token) {
        let next = self.peek().unwrap();
        if next == token {
            self.cursor += 1;
            return;
        }

        panic!("expected {token:?}, but got {next:?}");
    }

    fn expect_number(&mut self) -> i32 {
        let next = self.peek();

        if let Some(next) = &next {
            if matches!(next, Token::Num(_)) {
                if let Token::Num(n) = next {
                    self.cursor += 1;
                    return *n;
                }
            }
        }

        panic!("expected number, but got {next:?}");
    }

    fn program(&mut self) -> Vec<Node> {
        let mut code: Vec<Node> = vec![];

        while self.tokens[self.cursor] != Token::Eof {
            code.push(self.statement())
        }

        code
    }

    fn statement(&mut self) -> Node {
        let node = self.expression();
        self.expect(Token::Semicolon);

        node
    }

    fn expression(&mut self) -> Node {
        // self.assign()
        self.equality()
    }

    // fn assign(&mut self) -> Node {
    //     let mut node = self.equality();

    //     if self.consume(Token::Assign) {
    //         node = Node {
    //             kind: NodeKind::Assign,
    //             lhs: Some(Box::new(node)),
    //             rhs: Some(Box::new(self.assign())),
    //         }
    //     }

    //     node
    // }

    fn equality(&mut self) -> Node {
        let mut node = self.relational();

        loop {
            if self.consume(Token::Equal) {
                node = Node {
                    kind: NodeKind::Equal,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.relational())),
                }
            } else if self.consume(Token::NotEqual) {
                node = Node {
                    kind: NodeKind::NotEqual,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.relational())),
                }
            } else {
                return node;
            }
        }
    }

    fn relational(&mut self) -> Node {
        let mut node = self.add();

        loop {
            if self.consume(Token::LeftAngleBracket) {
                node = Node {
                    kind: NodeKind::LessThan,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.add())),
                }
            } else if self.consume(Token::LessThanOrEqual) {
                node = Node {
                    kind: NodeKind::LessThanOrEqual,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.add())),
                }
            } else if self.consume(Token::RightAngleBracket) {
                node = Node {
                    kind: NodeKind::LessThan,
                    rhs: Some(Box::new(self.add())),
                    lhs: Some(Box::new(node)),
                }
            } else if self.consume(Token::GreaterThanOrEqual) {
                node = Node {
                    kind: NodeKind::LessThanOrEqual,
                    rhs: Some(Box::new(self.add())),
                    lhs: Some(Box::new(node)),
                }
            } else {
                return node;
            }
        }
    }

    fn add(&mut self) -> Node {
        let mut node = self.mul();

        loop {
            if self.consume(Token::Plus) {
                node = Node {
                    kind: NodeKind::Add,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.mul())),
                }
            } else if self.consume(Token::Minus) {
                node = Node {
                    kind: NodeKind::Sub,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.mul())),
                }
            } else {
                return node;
            }
        }
    }

    fn mul(&mut self) -> Node {
        let mut node = self.unary();

        loop {
            if self.consume(Token::Star) {
                node = Node {
                    kind: NodeKind::Multiply,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.unary())),
                }
            } else if self.consume(Token::Slash) {
                node = Node {
                    kind: NodeKind::Div,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.unary())),
                }
            } else {
                return node;
            }
        }
    }

    fn unary(&mut self) -> Node {
        if self.consume(Token::Plus) {
            return self.unary();
        }

        if self.consume(Token::Minus) {
            return Node {
                kind: NodeKind::Sub,
                lhs: Some(Box::new(Node::new_num(0))),
                rhs: Some(Box::new(self.unary())),
            };
        }

        self.primary()
    }

    fn primary(&mut self) -> Node {
        if self.consume(Token::LeftParen) {
            let node = self.expression();
            self.expect(Token::RightParen);
            return node;
        }

        Node::new_num(self.expect_number())
    }
}

#[cfg(test)]
mod test {
    use crate::{parser, token::Token};

    #[test]
    fn test_number() {
        let tree = parser::Parser::new(vec![Token::Num(42), Token::Semicolon]).parse();
        assert_eq!(
            format!("{tree:?}",),
            "[Node { kind: Num(42), lhs: None, rhs: None }]"
        );
    }
}
