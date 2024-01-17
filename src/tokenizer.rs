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
                _ => {}
            }

            if c.is_ascii_digit() {
                let string: String = self.input.chars().skip(self.cursor).collect();
                let (n, len) = str_to_fromstr::<i32>(&string).unwrap();

                tokens.push(Token::Num(n));
                self.cursor += len;
                continue;
            }

            panic!(
                "トークナイズできません: {}",
                self.input.chars().skip(self.cursor).collect::<String>()
            )
        }

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
    fn test_single_digit_tokens() {
        // Plus
        let tokens = Tokenizer::new("+").tokenize();
        assert_eq!(format!("{:?}", tokens), format!("{:?}", vec![Token::Plus]));

        // Minus
        let tokens = Tokenizer::new("-").tokenize();
        assert_eq!(format!("{:?}", tokens), format!("{:?}", vec![Token::Minus]));

        // Star
        let tokens = Tokenizer::new("*").tokenize();
        assert_eq!(format!("{:?}", tokens), format!("{:?}", vec![Token::Star]));

        // LeftParen, RightParen
        let tokens = Tokenizer::new("()").tokenize();
        assert_eq!(
            format!("{:?}", tokens),
            format!("{:?}", vec![Token::LeftParen, Token::RightParen])
        );

        // LeftAngleBracket, RightAngleBracket
        let tokens = Tokenizer::new("<>").tokenize();
        assert_eq!(
            format!("{:?}", tokens),
            format!(
                "{:?}",
                vec![Token::LeftAngleBracket, Token::RightAngleBracket]
            )
        );
    }

    #[test]
    fn test_two_digit_tokens() {
        let tokens = Tokenizer::new("==").tokenize();
        assert_eq!(format!("{:?}", tokens), format!("{:?}", vec![Token::Equal]));

        let tokens = Tokenizer::new(">=").tokenize();
        assert_eq!(
            format!("{:?}", tokens),
            format!("{:?}", vec![Token::GreaterThanOrEqual])
        );

        let tokens = Tokenizer::new("<=").tokenize();
        assert_eq!(
            format!("{:?}", tokens),
            format!("{:?}", vec![Token::LessThanOrEqual])
        );

        let tokens = Tokenizer::new("!=").tokenize();
        assert_eq!(
            format!("{:?}", tokens),
            format!("{:?}", vec![Token::NotEqual])
        );
    }

    #[test]
    fn test_tokenizer() {
        let tokens = Tokenizer::new("1+5-(20*2)==10").tokenize();
        assert_eq!(format!("{:?}", tokens), "[Num(1), Plus, Num(5), Minus, LeftParen, Num(20), Star, Num(2), RightParen, Equal, Num(10)]")
    }
}
