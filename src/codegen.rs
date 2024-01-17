use std::io::Write;

use crate::node::{Node, NodeKind};

fn gen_address<W: Write>(w: &mut W, node: &Node) {
    if let NodeKind::Var(name) = node.kind {
        let offset = ((name as u8) - ('a' as u8) + 1) * 8;
        let _ = writeln!(w, "  lea -{offset}(%rbp), %rax");
        return;
    }

    panic!("ローカル変数ではありません: {node:?}");
}

fn gen_expression<W: Write>(w: &mut W, node: &Node) {
    match node.kind {
        NodeKind::Num(n) => {
            let _ = writeln!(w, "  mov ${n}, %rax");
            return;
        }
        NodeKind::Var(_) => {
            gen_address(w, node);
            let _ = writeln!(w, "  mov (%rax), %rax");
            return;
        }
        NodeKind::Assign => {
            gen_address(w, node.lhs.as_ref().unwrap());
            let _ = writeln!(w, "  push %rax");
            gen_expression(w, node.rhs.as_ref().unwrap());
            let _ = writeln!(w, "  pop %rdi");
            let _ = writeln!(w, "  mov %rax, (%rdi)");
            return;
        }
        _ => {}
    }

    println!("{node:?}");

    gen_expression(w, node.rhs.as_ref().unwrap());
    let _ = writeln!(w, "  push %rax");

    gen_expression(w, node.lhs.as_ref().unwrap());
    let _ = writeln!(w, "  pop %rdi");

    match node.kind {
        NodeKind::Add => {
            let _ = writeln!(w, "  add %rdi, %rax");
        }
        NodeKind::Sub => {
            let _ = writeln!(w, "  sub %rdi, %rax");
        }
        NodeKind::Multiply => {
            let _ = writeln!(w, "  imul %rdi, %rax");
        }
        NodeKind::Div => {
            let _ = writeln!(w, "  cqo");
            let _ = writeln!(w, "  idiv %rdi");
        }
        NodeKind::Equal | NodeKind::NotEqual | NodeKind::LessThan | NodeKind::LessThanOrEqual => {
            let _ = writeln!(w, "  cmp %rdi, %rax");

            match node.kind {
                NodeKind::Equal => {
                    let _ = writeln!(w, "  sete %al");
                }
                NodeKind::NotEqual => {
                    let _ = writeln!(w, "  setne %al");
                }
                NodeKind::LessThan => {
                    let _ = writeln!(w, "  setl %al");
                }
                NodeKind::LessThanOrEqual => {
                    let _ = writeln!(w, "  setle %al");
                }
                _ => {}
            }

            let _ = writeln!(w, "  movzb %al, %rax");
        }
        _ => {}
    }
}

pub fn codegen<W: Write>(w: &mut W, trees: Vec<Node>) {
    let _ = writeln!(w, "  .globl main");
    let _ = writeln!(w, "main:");

    let _ = writeln!(w, "  push %rbp");
    let _ = writeln!(w, "  mov %rsp, %rbp");
    let _ = writeln!(w, "  sub $208, %rsp");

    for tree in trees {
        gen_expression(w, &tree);
    }

    let _ = writeln!(w, "  mov %rbp, %rsp");
    let _ = writeln!(w, "  pop %rbp");
    let _ = writeln!(w, "  ret");
}
