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

fn run<W: Write>(w: &mut W, p: &str) {
    let tokens = tokenizer::tokenize(p);
    let tree = parser::Parser::new(&tokens).parse();

    let _ = writeln!(w, ".intel_syntax noprefix");
    let _ = writeln!(w, ".globl main");
    let _ = writeln!(w, "main:");

    generator::gen(w, tree);

    let _ = writeln!(w, "  pop rax");
    let _ = writeln!(w, "  ret");
}

#[cfg(test)]
mod test {
    use std::{
        fs::{self, File},
        io::Write,
        process::Command,
    };

    use crate::run;

    fn run_with_result(p: &str) -> i32 {
        let id = uuid::Uuid::new_v4();

        let mut asm_buf = Vec::<u8>::new();
        run(&mut asm_buf, &String::from(p));

        let mut asm_file = File::create(format!("{id}.s")).unwrap();
        asm_file.write_all(&asm_buf).unwrap();

        let _ = Command::new("cc")
            .arg("-o")
            .arg(format!("{id}"))
            .arg(format!("{id}.s"))
            .output()
            .unwrap();

        let out = Command::new(format!("./{id}"))
            .status()
            .unwrap()
            .code()
            .unwrap();

        let _ = fs::remove_file(format!("{id}"));
        let _ = fs::remove_file(format!("{id}.s"));

        out
    }

    #[test]
    fn test_basic_number() {
        assert_eq!(run_with_result(&String::from("0")), 0);
        assert_eq!(run_with_result(&String::from("42")), 42);
    }

    #[test]
    fn test_add_sub() {
        assert_eq!(run_with_result("5+20-4"), 21);
    }

    #[test]
    fn test_with_space() {
        assert_eq!(run_with_result(" 12 + 34 - 5 "), 41);
    }

    #[test]
    fn test_mul() {
        assert_eq!(run_with_result("5+6*7"), 47);
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

    #[test]
    fn test_eq() {
        assert_eq!(run_with_result(&String::from("0==1")), 0);
        assert_eq!(run_with_result(&String::from("42==42")), 1);
        assert_eq!(run_with_result(&String::from("0!=1")), 1);
        assert_eq!(run_with_result(&String::from("42!=42")), 0);
    }

    #[test]
    fn test_greater_than() {
        assert_eq!(run_with_result(&String::from("0<1")), 1);
        assert_eq!(run_with_result(&String::from("1<1")), 0);
        assert_eq!(run_with_result(&String::from("2<1")), 0);
    }

    #[test]
    fn test_greater_eq_than() {
        assert_eq!(run_with_result(&String::from("0<=1")), 1);
        assert_eq!(run_with_result(&String::from("1<=1")), 1);
        assert_eq!(run_with_result(&String::from("2<=1")), 0);
    }

    #[test]
    fn test_less_than() {
        assert_eq!(run_with_result(&String::from("1>0")), 1);
        assert_eq!(run_with_result(&String::from("1>1")), 0);
        assert_eq!(run_with_result(&String::from("1>2")), 0);
    }

    #[test]
    fn test_less_eq_than() {
        assert_eq!(run_with_result(&String::from("1>=0")), 1);
        assert_eq!(run_with_result(&String::from("1>=1")), 1);
        assert_eq!(run_with_result(&String::from("1>=2")), 0);
    }
}
