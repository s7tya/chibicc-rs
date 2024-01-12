mod tokenizer;

fn main() {
    let p = String::from("20+5");
    let a = tokenizer::tokenize(p);
    println!("{:#?}", a);
}
