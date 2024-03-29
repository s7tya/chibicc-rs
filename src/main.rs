use std::{
    env, fs,
    io::{stdout, Write},
    process::Command,
};

use node::Program;

mod codegen;
mod node;
mod parser;
mod token;
mod tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 && args.len() != 3 {
        panic!("引数の個数が正しくありません");
    }

    if args.len() == 3 {
        match args[1].as_str() {
            "run" => {
                println!("{}", run(&args[2]));
            }
            "tokenize" => {
                let tokens = tokenizer::Tokenizer::new(&args[2]).tokenize();
                println!("{:?}", tokens);
            }
            "parse" => {
                let tokens = tokenizer::Tokenizer::new(&args[2]).tokenize();
                let trees = parser::Parser::new(tokens).parse();
                println!("{:?}", trees);
            }
            "compile" => {
                write_asm(&mut stdout(), &args[2]);
            }
            _ => {}
        }
    } else {
        println!("{}", run(&args[1]));
    }
}

fn write_asm<W: Write>(w: &mut W, input: &str) {
    //
    // Tokenize
    //
    let mut tokenizer = tokenizer::Tokenizer::new(input);
    let tokens = tokenizer.tokenize();

    //
    // Parse
    //
    let mut parser = parser::Parser::new(tokens);
    let (body, locals) = parser.parse();

    //
    // Codegen
    //
    codegen::Generator::new().codegen(w, Program { body, locals });
}

fn run(input: &str) -> i32 {
    let mut asm_file = tempfile::NamedTempFile::new().expect("一時ファイルの作成に失敗しました");
    write_asm(&mut asm_file, input);

    let asm_file_path = asm_file.path();

    let asm_file_path_str = match asm_file_path.to_str() {
        Some(path) => path,
        None => panic!("アセンブリファイルのパスの取得に失敗しました"),
    };
    let binary_file_path_str = format!("{asm_file_path_str}.bin");

    let _ = Command::new("cc")
        .arg("-x")
        .arg("assembler")
        .arg("-o")
        .arg(&binary_file_path_str)
        .arg(asm_file_path_str)
        .output()
        .expect("アセンブリのコンパイルに失敗しました");

    let status_code: i32 = Command::new(&binary_file_path_str)
        .status()
        .unwrap()
        .code()
        .unwrap();

    let _ = asm_file.close();

    fs::remove_file(&binary_file_path_str).expect("バイナリファイルの削除に失敗しました");

    status_code
}

#[cfg(test)]
mod test {
    use crate::run;

    #[test]
    fn test_numbers() {
        assert_eq!(run("{ 0; }"), 0);
        assert_eq!(run("{ 42; }"), 42);
    }

    #[test]
    fn test_add_sub() {
        assert_eq!(run("{ 5+20-4; }"), 21);
    }

    #[test]
    fn test_with_space() {
        assert_eq!(run(" { 12 + 34 -  5 ;  }"), 41);
    }

    #[test]
    fn test_mul() {
        assert_eq!(run("{ 5+6*7; }"), 47);
    }

    #[test]
    fn test_primary() {
        assert_eq!(run("{5*(9-6);}"), 15);
        assert_eq!(run("{(3+5)/2;}"), 4);
    }

    #[test]
    fn test_unary() {
        assert_eq!(run("{-10+20;}"), 10);
    }

    #[test]
    fn test_eq() {
        assert_eq!(run("{0==1;}"), 0);
        assert_eq!(run("{42==42;}"), 1);
        assert_eq!(run("{0!=1;}"), 1);
        assert_eq!(run("{42!=42;}"), 0);
    }

    #[test]
    fn test_greater_than() {
        assert_eq!(run("{0<1;}"), 1);
        assert_eq!(run("{1<1;}"), 0);
        assert_eq!(run("{2<1;}"), 0);
    }

    #[test]
    fn test_greater_eq_than() {
        assert_eq!(run("{0<=1;}"), 1);
        assert_eq!(run("{1<=1;}"), 1);
        assert_eq!(run("{2<=1;}"), 0);
    }

    #[test]
    fn test_less_than() {
        assert_eq!(run("{1>0;}"), 1);
        assert_eq!(run("{1>1;}"), 0);
        assert_eq!(run("{1>2;}"), 0);
    }

    #[test]
    fn test_less_eq_than() {
        assert_eq!(run("{1>=0;}"), 1);
        assert_eq!(run("{1>=1;}"), 1);
        assert_eq!(run("{1>=2;}"), 0);
    }

    #[test]
    fn test_multiple_statements() {
        assert_eq!(run("{1; 2; 3;}"), 3);
    }

    #[test]
    fn test_single_letter_var() {
        assert_eq!(run("{a = 5;}"), 5);
        assert_eq!(run("{k = 40;}"), 40);
        assert_eq!(run("{a = 12; b = 3; a * b;}"), 36);
    }

    #[test]
    fn test_multi_letter_var() {
        assert_eq!(run("{test = 5;}"), 5);
        assert_eq!(run("{returnx = 5;}"), 5);
        assert_eq!(run("{r906 = 96;}"), 96);
        assert_eq!(run("{r_906 = 5;}"), 5);
        assert_eq!(run("{pi = 3; r = 5; r * r * pi;}"), 75);
    }

    #[test]
    fn test_return_statement() {
        assert_eq!(run("{42; return 30; 4;}"), 30);
        assert_eq!(run("{return a = 5;}"), 5);
        assert_eq!(run("{a = 10; b = 12; return a * b - b; 42;}"), 108);
    }

    #[test]
    fn test_null_statement() {
        assert_eq!(run("{ ;;; return 5; }"), 5);
    }

    #[test]
    fn test_if_statement() {
        assert_eq!(
            run("
                { 
                    if (1==2) { 
                        a = 5; 
                    } else { 
                        a = 10; 
                    }

                    if (2==2) { 
                        b = 10; 
                    } else { 
                        b = 0; 
                    } 

                    return a*b; 
                }
                "),
            100
        );
    }
}
