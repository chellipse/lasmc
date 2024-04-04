use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

// local module
use crate::parser::Expression;
use asm_lisp::{error, system, warning};

// enum Type {
    // Byte,
    // Short,
// }

enum Op {
    False,
    Add,
    Sub,
    Mul,
}

fn list(ops: &mut String, li1: Vec<Expression>) {
    let mut iter = li1.into_iter();

    let e1 = iter.next();
    let op = match e1 {
        None => {Op::False},
        Some(Expression::Atom(s)) => {
            match s.as_str() {
                "+" => {Op::Add},
                "-" => {Op::Sub},
                "*" => {Op::Mul},
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
                        ["movl $", ", %eax"],
                        ["addl $", ", %eax"]
                    ];
                    let mut offset: i32 = 0;
                    for (i, item) in iter.enumerate() {
                        match item {
                            Expression::Atom(s) => {
                                ops.push_str(format!("    {}{}{}\n",
                                    lines[i][0],
                                    s,
                                    lines[i][1]).as_str());
                            },
                            Expression::List(li2) => {
                                list(ops, li2);
                                offset += 4;
                                ops.push_str(format!("    movl %eax, -{}(%rsp)\n", offset).as_str());
                            },
                        };
                    }
                }
                i => {error!("Unsupported number of operands for `+` operator: {}", i)}
            }
        },
        Op::Sub => {todo!()},
        Op::Mul => {todo!()},
    }
}

pub fn encode(cst: Vec<Expression>, _filename: String) {

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
    pushq %rbp
    movq %rsp, %rbp
	subq	$0, %rsp

");
        buf.append(&mut d1);
    }

    for item in cst.into_iter() {
        match item {
            Expression::List(v) => {
                let mut ops = String::new();
                list(&mut ops, v);
                let mut vec = Vec::from(ops);
                println!("{:?}", &vec);
                buf.append(&mut vec);
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

	addq	$0, %rsp
    popq  %rbp
    retq
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

