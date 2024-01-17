use std::{
    env,
    io::{stdout, Write},
    process::Command,
};

mod node;
mod token;
mod tokenizer;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("引数の個数が正しくありません");
    }

    write_asm(&mut stdout(), &args[2]);
}

fn write_asm<W: Write>(w: &mut W, input: &str) {
    //
    // Tokenize
    //
    let mut tokenizer = tokenizer::Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    // let _ = writeln!(w, "{:?}", tokens);

    //
    // Parse
    //

    //
    // Codegen
    //
    let _ = writeln!(w, ".intel_syntax noprefix");
    let _ = writeln!(w, ".globl main");
    let _ = writeln!(w, "main:");

    let _ = writeln!(w, "  pop rax");
    let _ = writeln!(w, "  ret");
}

fn run(input: &str) -> i32 {
    let mut asm_file = tempfile::NamedTempFile::new().expect("一時ファイルの作成に失敗しました");
    write_asm(&mut asm_file, input);

    let binary_file = tempfile::NamedTempFile::new().expect("一次ファイルの作成に失敗しました");

    let asm_file_path = &asm_file.path();
    let binary_file_path = binary_file.path();

    let _ = Command::new("cc")
        .arg("-o")
        .arg(binary_file_path)
        .arg(asm_file_path);

    let status_code: i32 = Command::new(binary_file_path)
        .status()
        .unwrap()
        .code()
        .unwrap();

    status_code
}

#[cfg(test)]
mod test {
    use crate::run;

    #[test]
    fn test() {
        assert_eq!(run("5 + 2"), 7);
    }
}
