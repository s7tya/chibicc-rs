use std::{process::exit, str::FromStr};

#[derive(Debug)]
enum TokenKind {
    Reserved,
    Num(i32),
    Eof,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    input: String,
}

pub fn tokenize(user_input: String) -> Vec<Token> {
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
                    input: user_input.clone(),
                });
                p = p.split_off(1);
                continue;
            }
            _ => {}
        }

        if c.is_ascii_digit() {
            let (n, str) = str_to_fromstr::<i32>(&p).unwrap();
            p = String::from(str);
            tokens.push(Token {
                kind: TokenKind::Num(n),
                input: user_input.clone(),
            });
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
