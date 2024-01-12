use std::env;

mod generator;
mod parser;
mod tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("引数の個数が間違っています");
    }

    let tokens = tokenizer::tokenize(&args[1]);
    let tree = parser::Parser::new(&tokens).parse();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    generator::gen(Box::new(tree));

    println!("  pop rax");
    println!("  ret");
}
