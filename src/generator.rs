use std::io::Write;

use crate::parser::{Node, NodeKind};

pub fn gen<W: Write>(w: &mut W, node: Box<Node>) {
    if matches!(node.kind, NodeKind::Num(_)) {
        if let NodeKind::Num(n) = node.kind {
            writeln!(w, "  push {}", n);
            return;
        }
    }

    gen(w, node.lhs.unwrap());
    gen(w, node.rhs.unwrap());

    writeln!(w, "  pop rdi");
    writeln!(w, "  pop rax");

    match node.kind {
        NodeKind::Add => {
            writeln!(w, "  add rax, rdi");
        }
        NodeKind::Sub => {
            writeln!(w, "  sub rax, rdi");
        }
        NodeKind::Mul => {
            writeln!(w, "  imul rax, rdi");
        }
        NodeKind::Div => {
            writeln!(w, "  cqo");
            writeln!(w, "  idiv rdi");
        }
        NodeKind::Num(_) => {}
    }

    writeln!(w, "  push rax");
}
