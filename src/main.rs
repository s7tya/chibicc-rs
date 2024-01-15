use std::{env, io::Write};

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
// fn run_with_result(p: &str) -> i32 {
//     let mut asm_buf = Vec::<u8>::new();
//     run(&mut asm_buf, &String::from(p));

//     let mut asm_file = File::create("tmp.s").unwrap();
//     let _ = asm_file.write_all(&asm_buf).unwrap();

//     let _ = Command::new("cc")
//         .arg("-o")
//         .arg("tmp")
//         .arg("tmp.s")
//         .output()
//         .unwrap();

//     let out = Command::new("./tmp").status().unwrap().code().unwrap();
//     out
// }

#[cfg(test)]
mod test {
    use std::{
        fs::{self, File},
        io::Write,
        process::Command,
    };

    use crate::run;

    fn clean() {
        let _ = fs::remove_file("tmp");
        let _ = fs::remove_file("tmp.s");
    }

    fn run_with_result(p: &str) -> i32 {
        clean();

        let mut asm_buf = Vec::<u8>::new();
        run(&mut asm_buf, &String::from(p));

        let mut asm_file = File::create("tmp.s").unwrap();
        let _ = asm_file.write_all(&asm_buf).unwrap();

        let _ = Command::new("cc")
            .arg("-o")
            .arg("tmp")
            .arg("tmp.s")
            .output()
            .unwrap();

        let out = Command::new("./tmp").status().unwrap().code().unwrap();
        out
    }

    #[test]
    fn test_basic_number() {
        assert_eq!(run_with_result(&String::from("0")), 0);
        assert_eq!(run_with_result(&String::from("42")), 42);
    }

    #[test]
    fn test_add_sub() {
        assert_eq!(run_with_result(&String::from("5+20-4")), 21);
    }

    #[test]
    fn test_with_space() {
        assert_eq!(run_with_result(&String::from(" 12 + 34 - 5 ")), 41);
    }

    #[test]
    fn test_mul() {
        assert_eq!(run_with_result(&String::from("5+6*7")), 47);
    }

    #[test]
    fn test_primary() {
        assert_eq!(run_with_result(&String::from("5*(9-6)")), 15);
        assert_eq!(run_with_result(&String::from("(3+5)/2")), 4);
    }

    #[test]
    fn test_unary() {
        assert_eq!(run_with_result(&String::from("-10+20")), 10);
    }
}
