use std::{
    env,
    io::{stdout, Write},
};

mod generator;
mod parser;
mod tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("引数の個数が間違っています");
    }

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    run(&mut stdout, &args[1]);
}

fn run<W: Write>(w: &mut W, p: &String) {
    let tokens = tokenizer::tokenize(p);
    let tree = parser::Parser::new(&tokens).parse();

    writeln!(w, ".intel_syntax noprefix");
    writeln!(w, ".globl main");
    writeln!(w, "main:");

    generator::gen(w, Box::new(tree));

    writeln!(w, "  pop rax");
    writeln!(w, "  ret");
}

#[cfg(test)]
mod test {
    use crate::run;

    #[test]
    fn test_asm() {
        let mut buf = Vec::<u8>::new();
        run(&mut buf, &String::from("20*5+2"));
    }
}
