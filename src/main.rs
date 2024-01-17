use std::{env, fs, io::stdout, process::Command};

use codegen::write_asm;

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

    if args.len() == 3 && args[1] == "run" {
        println!("{}", run(&args[2]));
    } else {
        write_asm(&mut stdout(), &args[1]);
    }
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

// #[cfg(test)]
// mod test {
//     use crate::run;

//     #[test]
//     fn test() {
//         assert_eq!(run("5 + 2"), 7);
//     }
// }
