use std::{
    env, fs,
    io::{stdout, Write},
    os::unix::fs::PermissionsExt,
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

    // let _ = run("1+5");
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

    let _ = writeln!(w, "  push 2");
    let _ = writeln!(w, "  push 5");
    let _ = writeln!(w, "   pop rdi");
    let _ = writeln!(w, "  pop rax");
    let _ = writeln!(w, "  add rax, rdi");
    let _ = writeln!(w, "  push rax");

    let _ = writeln!(w, "  pop rax");
    let _ = writeln!(w, "  ret");
}

fn run(input: &str) -> i32 {
    let mut asm_file = tempfile::NamedTempFile::new().expect("一時ファイルの作成に失敗しました");
    write_asm(&mut asm_file, input);

    let binary_file = tempfile::NamedTempFile::new().expect("一次ファイルの作成に失敗しました");

    let binary_file_path = binary_file.path();
    let asm_file_path = asm_file.path();

    let metadata = fs::metadata(binary_file_path).unwrap();
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(binary_file_path, permissions).unwrap();

    let _ = Command::new("cc")
        .arg("-x")
        .arg("assembler")
        .arg("-o")
        .arg(binary_file_path.to_str().unwrap())
        .arg(asm_file_path.to_str().unwrap())
        .output()
        .expect("アセンブリのコンパイルに失敗しました");

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
