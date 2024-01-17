use std::io::Write;

use crate::node::{Node, NodeKind};

pub fn gen<W: Write>(w: &mut W, node: &Node) {
    if matches!(node.kind, NodeKind::Num(_)) {
        if let NodeKind::Num(n) = node.kind {
            let _ = writeln!(w, "  mov ${n}, %rax");
            return;
        }
    }

    gen(w, node.rhs.as_ref().unwrap());
    let _ = writeln!(w, "  push %rax");
    gen(w, node.lhs.as_ref().unwrap());
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
        // NodeKind::Assign => {}
        NodeKind::Num(_) => {}
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
    }
}

pub fn codegen<W: Write>(w: &mut W, trees: Vec<Node>) {
    let _ = writeln!(w, "  .globl main");
    let _ = writeln!(w, "main:");

    for tree in trees {
        gen(w, &tree);
    }

    let _ = writeln!(w, "  ret");
}
