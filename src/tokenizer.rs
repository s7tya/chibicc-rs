use std::{process::exit, str::FromStr};

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Reserved,
    Num(i32),
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub str: String,
}

pub fn tokenize(user_input: &String) -> Vec<Token> {
    let mut p = user_input.clone();
    let mut tokens: Vec<Token> = vec![];

    while let Some(c) = p.chars().next() {
        if c.is_ascii_whitespace() {
            p = p.split_off(1);
            continue;
        }

        match c {
            '+' | '-' | '*' | '(' | ')' => {
                tokens.push(Token {
                    kind: TokenKind::Reserved,
                    str: p.clone(),
                });
                p = p.split_off(1);
                continue;
            }
            _ => {}
        }

        if c.is_ascii_digit() {
            let (n, str) = str_to_fromstr::<i32>(&p).unwrap();
            tokens.push(Token {
                kind: TokenKind::Num(n),
                str: p.clone(),
            });
            p = String::from(str);
            continue;
        }

        eprintln!("トークナイズできません: {}", p);
        exit(1);
    }

    tokens
}

fn str_to_fromstr<F: FromStr>(str: &str) -> Result<(F, &str), F::Err> {
    let index = str
        .bytes()
        .position(|byte| !byte.is_ascii_digit())
        .unwrap_or(str.len());

    let (digit_part, remaining_part) = str.split_at(index);

    digit_part.parse().map(|value| (value, remaining_part))
}
