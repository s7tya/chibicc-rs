mod token;
mod tokenizer;
fn main() {
    let mut tokenizer = tokenizer::Tokenizer::new("1+5-(20*2)==10");
    let tokens = tokenizer.tokenize();

    println!("{:?}", tokens);
}
