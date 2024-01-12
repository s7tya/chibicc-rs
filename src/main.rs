use std::env;

mod parser;
mod tokenizer;

fn main() {
    // let p = String::from("20+5*20+(5+4)*2");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("");
    }

    let tokens = tokenizer::tokenize(&args[2]);
    let tree = parser::Parser::new(&tokens).parse();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    // TODO: generator

    println!("  pop rax");
    println!("  ret");
}
