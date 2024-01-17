use core::panic;
use std::str::FromStr;

use crate::token::Token;

pub struct Tokenizer {
    input: String,
    cursor: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Tokenizer {
            input: String::from(input),
            cursor: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while let Some(c) = self.input.chars().nth(self.cursor) {
            if c.is_ascii_whitespace() {
                self.cursor += 1;
                continue;
            }

            match self.peek(2).as_str() {
                ">=" => {
                    tokens.push(Token::GreaterThanOrEqual);
                    self.cursor += 2;
                    continue;
                }
                "<=" => {
                    tokens.push(Token::LessThanOrEqual);
                    self.cursor += 2;
                    continue;
                }
                "==" => {
                    tokens.push(Token::Equal);
                    self.cursor += 2;
                    continue;
                }
                "!=" => {
                    tokens.push(Token::NotEqual);
                    self.cursor += 2;
                    continue;
                }
                _ => {}
            }

            match self.peek(1).as_str() {
                "+" => {
                    tokens.push(Token::Plus);
                    self.cursor += 1;
                    continue;
                }
                "-" => {
                    tokens.push(Token::Minus);
                    self.cursor += 1;
                    continue;
                }
                "*" => {
                    tokens.push(Token::Star);
                    self.cursor += 1;
                    continue;
                }
                "/" => {
                    tokens.push(Token::Slash);
                    self.cursor += 1;
                    continue;
                }
                "(" => {
                    tokens.push(Token::LeftParen);
                    self.cursor += 1;
                    continue;
                }
                ")" => {
                    tokens.push(Token::RightParen);
                    self.cursor += 1;
                    continue;
                }
                "<" => {
                    tokens.push(Token::LeftAngleBracket);
                    self.cursor += 1;
                    continue;
                }
                ">" => {
                    tokens.push(Token::RightAngleBracket);
                    self.cursor += 1;
                    continue;
                }
                ";" => {
                    tokens.push(Token::Semicolon);
                    self.cursor += 1;
                    continue;
                }
                "=" => {
                    tokens.push(Token::Assign);
                    self.cursor += 1;
                    continue;
                }
                _ => {}
            }

            if c.is_ascii_digit() {
                let string: String = self.input.chars().skip(self.cursor).collect();
                let (n, len) = str_to_fromstr::<i32>(&string).unwrap();

                tokens.push(Token::Num(n));
                self.cursor += len;
                continue;
            }

            if c.is_ascii_alphabetic() {
                if let Some(name) = self.input.chars().nth(self.cursor) {
                    tokens.push(Token::Ident(name));
                    self.cursor += 1;
                    continue;
                }
            }

            panic!(
                "トークナイズできません: {}",
                self.input.chars().skip(self.cursor).collect::<String>()
            )
        }

        tokens.push(Token::Eof);

        tokens
    }

    fn peek(&self, n: usize) -> String {
        self.input.chars().skip(self.cursor).take(n).collect()
    }
}

fn str_to_fromstr<F: FromStr>(str: &str) -> Result<(F, usize), F::Err> {
    let index = str
        .bytes()
        .position(|byte| !byte.is_ascii_digit())
        .unwrap_or(str.len());

    let (digit_part, _) = str.split_at(index);

    digit_part.parse().map(|value| (value, digit_part.len()))
}

#[cfg(test)]
mod test {
    use crate::token::Token;

    use super::Tokenizer;

    #[test]
    fn test_space_between_tokens() {
        let tokens = Tokenizer::new("5 5 5").tokenize();
        assert_eq!(
            format!("{:?}", tokens),
            format!(
                "{:?}",
                vec![Token::Num(5), Token::Num(5), Token::Num(5), Token::Eof]
            )
        );
    }

    #[test]
    fn test_single_digit_tokens() {
        // Plus
        let tokens = Tokenizer::new("+").tokenize();
        assert_eq!(
            format!("{:?}", tokens),
            format!("{:?}", vec![Token::Plus, Token::Eof])
        );

        // Minus
        let tokens = Tokenizer::new("-").tokenize();
        assert_eq!(
            format!("{:?}", tokens),
            format!("{:?}", vec![Token::Minus, Token::Eof])
        );

        // Star
        let tokens = Tokenizer::new("*").tokenize();
        assert_eq!(
            format!("{:?}", tokens),
            format!("{:?}", vec![Token::Star, Token::Eof])
        );

        // LeftParen, RightParen
        let tokens = Tokenizer::new("()").tokenize();
        assert_eq!(
            format!("{:?}", tokens),
            format!(
                "{:?}",
                vec![Token::LeftParen, Token::RightParen, Token::Eof]
            )
        );

        // LeftAngleBracket, RightAngleBracket
        let tokens = Tokenizer::new("<>").tokenize();
        assert_eq!(
            format!("{:?}", tokens),
            format!(
                "{:?}",
                vec![
                    Token::LeftAngleBracket,
                    Token::RightAngleBracket,
                    Token::Eof
                ]
            )
        );
    }

    #[test]
    fn test_two_digit_tokens() {
        let tokens = Tokenizer::new("==").tokenize();
        assert_eq!(
            format!("{:?}", tokens),
            format!("{:?}", vec![Token::Equal, Token::Eof])
        );

        let tokens = Tokenizer::new(">=").tokenize();
        assert_eq!(
            format!("{:?}", tokens),
            format!("{:?}", vec![Token::GreaterThanOrEqual, Token::Eof])
        );

        let tokens = Tokenizer::new("<=").tokenize();
        assert_eq!(
            format!("{:?}", tokens),
            format!("{:?}", vec![Token::LessThanOrEqual, Token::Eof])
        );

        let tokens = Tokenizer::new("!=").tokenize();
        assert_eq!(
            format!("{:?}", tokens),
            format!("{:?}", vec![Token::NotEqual, Token::Eof])
        );
    }

    #[test]
    fn test_tokenizer() {
        let tokens = Tokenizer::new("1+5-(20*2)==10").tokenize();
        assert_eq!(
            format!("{:?}", tokens),
            format!(
                "{:?}",
                vec![
                    Token::Num(1),
                    Token::Plus,
                    Token::Num(5),
                    Token::Minus,
                    Token::LeftParen,
                    Token::Num(20),
                    Token::Star,
                    Token::Num(2),
                    Token::RightParen,
                    Token::Equal,
                    Token::Num(10),
                    Token::Eof,
                ]
            )
        )
    }
}
