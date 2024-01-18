use std::{collections::HashMap, io::Write};

use crate::node::{Node, NodeKind, Program};

pub struct Generator {
    locals: HashMap<String, i32>,
    stack_size: i32,
    count: i32,
}

impl Generator {
    pub fn new() -> Self {
        Generator {
            locals: HashMap::new(),
            stack_size: 0,
            count: 0,
        }
    }

    fn gen_address<W: Write>(&mut self, w: &mut W, node: &Node) {
        if let NodeKind::Var(name) = &node.kind {
            let _ = writeln!(w, "  lea {}(%rbp), %rax", self.locals.get(name).unwrap());
            return;
        }

        panic!("ローカル変数ではありません: {node:?}");
    }

    fn gen_statement<W: Write>(&mut self, w: &mut W, node: &Node) {
        match &node.kind {
            NodeKind::Block(nodes) => {
                for node in nodes {
                    self.gen_statement(w, node);
                }
            }
            NodeKind::Return => {
                self.gen_expression(w, node.lhs.as_ref().unwrap());

                let _ = writeln!(w, "  jmp .L.return");
            }
            NodeKind::If(condition, then, els) => {
                self.count += 1;
                self.gen_expression(w, condition);
                let _ = writeln!(w, "  cmp $0, %rax");
                let _ = writeln!(w, "  je  .L.else.{}", self.count);
                self.gen_statement(w, then);
                let _ = writeln!(w, "  jmp .L.end.{}", self.count);
                let _ = writeln!(w, ".L.else.{}:", self.count);
                if let Some(els) = els.as_ref() {
                    self.gen_statement(w, els)
                }
                let _ = writeln!(w, ".L.end.{}:", self.count);
            }
            NodeKind::ExpressionStatement => self.gen_expression(w, node.lhs.as_ref().unwrap()),
            _ => {}
        }
    }

    fn gen_expression<W: Write>(&mut self, w: &mut W, node: &Node) {
        match node.kind {
            NodeKind::Num(n) => {
                let _ = writeln!(w, "  mov ${n}, %rax");
                return;
            }
            NodeKind::Var(_) => {
                self.gen_address(w, node);
                let _ = writeln!(w, "  mov (%rax), %rax");
                return;
            }
            NodeKind::Assign => {
                self.gen_address(w, node.lhs.as_ref().unwrap());
                let _ = writeln!(w, "  push %rax");
                self.gen_expression(w, node.rhs.as_ref().unwrap());
                let _ = writeln!(w, "  pop %rdi");
                let _ = writeln!(w, "  mov %rax, (%rdi)");
                return;
            }
            _ => {}
        }

        self.gen_expression(w, node.rhs.as_ref().unwrap());
        let _ = writeln!(w, "  push %rax");

        self.gen_expression(w, node.lhs.as_ref().unwrap());
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
            NodeKind::Equal
            | NodeKind::NotEqual
            | NodeKind::LessThan
            | NodeKind::LessThanOrEqual => {
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

    fn assign_lvar_offset(&mut self, function: &Program) {
        let mut offset = 0;
        for var in &function.locals {
            offset += 8;
            self.locals.insert(var.clone(), -offset);
        }
        self.stack_size = align_to(offset, 16);
    }

    pub fn codegen<W: Write>(&mut self, w: &mut W, function: Program) {
        self.assign_lvar_offset(&function);

        let _ = writeln!(w, "  .globl main");
        let _ = writeln!(w, "main:");

        let _ = writeln!(w, "  push %rbp");
        let _ = writeln!(w, "  mov %rsp, %rbp");
        let _ = writeln!(w, "  sub ${}, %rsp", self.stack_size);

        self.gen_statement(w, &function.body);

        let _ = writeln!(w, ".L.return:");
        let _ = writeln!(w, "  mov %rbp, %rsp");
        let _ = writeln!(w, "  pop %rbp");
        let _ = writeln!(w, "  ret");
    }
}

fn align_to(n: i32, align: i32) -> i32 {
    (n + align - 1) / align * align
}
