use std::io::Write;

use crate::{
    node::{Node, NodeKind},
    parser, tokenizer,
};

pub fn gen<W: Write>(w: &mut W, node: &Node) {
    if matches!(node.kind, NodeKind::Num(_)) {
        if let NodeKind::Num(n) = node.kind {
            let _ = writeln!(w, "  push {}", n);
            return;
        }
    }

    gen(w, node.lhs.as_ref().unwrap());
    gen(w, node.rhs.as_ref().unwrap());

    let _ = writeln!(w, "  pop rdi");
    let _ = writeln!(w, "  pop rax");

    match node.kind {
        NodeKind::Add => {
            let _ = writeln!(w, "  add rax, rdi");
        }
        NodeKind::Sub => {
            let _ = writeln!(w, "  sub rax, rdi");
        }
        NodeKind::Multiply => {
            let _ = writeln!(w, "  imul rax, rdi");
        }
        NodeKind::Div => {
            let _ = writeln!(w, "  cqo");
            let _ = writeln!(w, "  idiv rdi");
        }
        // NodeKind::Assign => {}
        NodeKind::Num(_) => {}
        NodeKind::Equal | NodeKind::NotEqual | NodeKind::LessThan | NodeKind::LessThanOrEqual => {
            let _ = writeln!(w, "  cmp rax, rdi");

            match node.kind {
                NodeKind::Equal => {
                    let _ = writeln!(w, "  sete al");
                }
                NodeKind::NotEqual => {
                    let _ = writeln!(w, "  setne al");
                }
                NodeKind::LessThan => {
                    let _ = writeln!(w, "  setl al");
                }
                NodeKind::LessThanOrEqual => {
                    let _ = writeln!(w, "  setle al");
                }
                _ => {}
            }

            let _ = writeln!(w, "  movzb rax, al");
        }
    }

    let _ = writeln!(w, "  push rax");
}

pub fn write_asm<W: Write>(w: &mut W, input: &str) {
    //
    // Tokenize
    //
    let mut tokenizer = tokenizer::Tokenizer::new(input);
    let tokens = tokenizer.tokenize();

    //
    // Parse
    //
    let mut parser = parser::Parser::new(tokens);
    let trees = parser.parse();

    //
    // Codegen
    //
    let _ = writeln!(w, ".intel_syntax noprefix");
    let _ = writeln!(w, ".globl main");
    let _ = writeln!(w, "main:");

    for tree in trees {
        gen(w, &tree);
    }

    let _ = writeln!(w, "  ret");
}
