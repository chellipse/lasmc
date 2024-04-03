use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

// local module
use crate::parser::Expression;
use asm_lisp::{error, system, warning};

enum Type {
    Byte,
    Short,
}

enum Op {
    False,
    Add,
}

fn list(buf: &mut Vec<u8>, li1: Vec<Expression>) {
    let mut iter = li1.into_iter();
    dbg!(&iter);

    let e1 = iter.next();
    let op = match e1 {
        None => {Op::False},
        Some(Expression::Atom(s)) => {
            match s.as_str() {
                "+" => {Op::Add},
                invalid => {
                    error!("Invalid Operator `{}`", invalid);
                    exit(1)
                },
            }
        },
        Some(Expression::List(v)) => {
            error!("List found where keyword should be! `{:?}`", v);
            exit(1)
        },
    };

    let len = iter.len();
    match op {
        Op::False => {todo!()},
        Op::Add => {
            match len {
                2 => {
                    let lines: [[&str;2];2] = [
                        ["movq $", ", %rax\n"],
                        ["add $", ", %rax\n"]
                    ];
                    let mut ops = String::new();
                    for (i, item) in iter.enumerate() {
                        let val = match item {
                            Expression::Atom(s) => Some(s),
                            Expression::List(li2) => {
                                list(buf, li2);
                                None
                            },
                        };
                        if let Some(s) = val {
                            ops.push_str(format!("    {}{}{}",
                                lines[i][0],
                                s,
                                lines[i][1]).as_str());
                        }
                    }
                    let mut vec = Vec::from(ops);
                    buf.append(&mut vec);
                }
                i => {error!("Unsupported number of operands for `+` operator: {}", i)}
            }
        },
    }
}

pub fn encode(cst: Vec<Expression>, _filename: String) {
    // dbg!(&cst);

    let full_name = format!("ignore/asm.s");
    let mut file = match File::create(&full_name) {
        Ok(f) => f,
        Err(e) => {
            error!("creating {} resulted in {}", full_name, e);
            std::process::exit(1);
        },
    };

    let mut buf: Vec<u8> = Vec::new();
    {
        let mut d1 = Vec::from(".global _start

.text

_start:
    movq %rsp, %rbp

");
        buf.append(&mut d1);
    }

    for item in cst.into_iter() {
        match item {
            Expression::List(v) => {
                list(&mut buf, v);
            },
            Expression::Atom(s) => {
                warning!("Top level Atom `{}` ignored.", s);
            },
        }
    }

    {
        let mut d1 = Vec::from("
    movq $60, %rax
    movq $0, %rdi
    syscall

    pop %rbp
");
        buf.append(&mut d1);
    }

    match file.write_all(&buf) {
        Ok(_) => {},
        Err(e) => {
            error!("writing to {} returned {}", full_name, e);
        },
    };

    system!("wrote `{}` to `{:?}`", full_name, String::from_utf8(buf));
}

