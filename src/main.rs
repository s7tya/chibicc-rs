mod parser;
mod tokenizer;

fn main() {
    let p = String::from("20+5*20+(5+4)*2");
    let tokens = tokenizer::tokenize(p);
    println!("tokens: {:?}", tokens);

    let tree = parser::Parser::new(&tokens).expr();
    println!("tree {:#?}", tree);
}
