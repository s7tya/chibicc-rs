use std::io::Write;

use crate::parser::{Node, NodeKind};

pub fn gen<W: Write>(w: &mut W, node: Node) {
    if matches!(node.kind, NodeKind::Num(_)) {
        if let NodeKind::Num(n) = node.kind {
            let _ = writeln!(w, "  push {}", n);
            return;
        }
    }

    gen(w, *node.lhs.unwrap());
    gen(w, *node.rhs.unwrap());

    let _ = writeln!(w, "  pop rdi");
    let _ = writeln!(w, "  pop rax");

    match node.kind {
        NodeKind::Add => {
            let _ = writeln!(w, "  add rax, rdi");
        }
        NodeKind::Sub => {
            let _ = writeln!(w, "  sub rax, rdi");
        }
        NodeKind::Mul => {
            let _ = writeln!(w, "  imul rax, rdi");
        }
        NodeKind::Div => {
            let _ = writeln!(w, "  cqo");
            let _ = writeln!(w, "  idiv rdi");
        }
        NodeKind::Eq => {
            let _ = writeln!(w, "  cmp rax, rdi");
            let _ = writeln!(w, "  sete al");
            let _ = writeln!(w, "  movzb rax, al");
        }
        NodeKind::Ne => {
            let _ = writeln!(w, "  cmp rax, rdi");
            let _ = writeln!(w, "  setne al");
            let _ = writeln!(w, "  movzb rax, al");
        }
        NodeKind::Lt => {
            let _ = writeln!(w, "  cmp rax, rdi");
            let _ = writeln!(w, "  setl al");
            let _ = writeln!(w, "  movzb rax, al");
        }
        NodeKind::Le => {
            let _ = writeln!(w, "  cmp rax, rdi");
            let _ = writeln!(w, "  setle al");
            let _ = writeln!(w, "  movzb rax, al");
        }
        NodeKind::Num(_) => {}
    }

    let _ = writeln!(w, "  push rax");
}
