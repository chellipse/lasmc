use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use std::collections::HashMap;

// local module
use crate::parser::Expression;

#[allow(unused_imports)]
use lasmc::{error, system, warning};

#[allow(dead_code)]
enum Op {
    False,
    Add,
    Sub,
    Mul,
    Print,
    Syscall,
    Alloc,
    U32,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Type {
    Buf(i32),
    U32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Variable {
    name: String,
    stack_pos: i32,
    t: Type,
}

macro_rules! code {
    ($ops:ident, $fmt:expr) => (
        $ops.push_str($fmt);
    );
    ($ops:ident, $fmt:expr, $($arg:tt)*) => (
        $ops.push_str(format!($fmt, $($arg)*).as_str());
    )
}

#[derive(Debug)]
struct CompState {
    ops: String,
    cst: Vec<Expression>,
    offset: i32,
    state: HashMap<String, Variable>,
}

fn eval(ops: &mut String, li1: Vec<Expression>, offset: &mut i32) {
    let mut iter = li1.into_iter();

    let e1 = iter.next();
    let op = match e1 {
        Some(Expression::Atom(s)) => {
            match s.as_str() {
                "+" => {Op::Add},
                "-" => {Op::Sub},
                "*" => {Op::Mul},
                "syscall" => {Op::Syscall},
                "alloc" => {Op::Alloc},
                "u32" => {Op::U32},
                invalid => {
                    error!("Invalid Operator `{}`", invalid);
                    exit(1)
                },
            }
        },
        None => {Op::False},
        Some(Expression::List(v)) => {
            error!("List found where keyword should be! `{:?}`", v);
            exit(1)
        },
    };

    let asm = match op {
        Op::Add =>   {"addl"},
        Op::Sub =>   {"subl"},
        Op::Mul =>   {"imul"},
        _ => {""},
    };



    let mut buf_size: i32 = 0;

    let mut map: HashMap<String, Variable> = HashMap::new();

    for (i, item) in iter.enumerate() {
        match item {
            Expression::Atom(s) => {
                match op {
                    Op::Syscall => {
                        match i {
                            0 => {code!(ops, "    movq {}, %rax\n", s);},
                            1 => {code!(ops, "    movq {}, %rdi\n", s);},
                            2 => {code!(ops, "    leaq {}, %rsi\n", s);},
                            3 => {code!(ops, "    movq {}, %rdx\n", s);},
                            ignore => {warning!("Arg ignored `{}`", ignore)},
                        }
                    },
                    Op::Alloc => {
                        match i {
                            0 => {
                                buf_size = s.parse::<i32>().unwrap();
                                *offset += buf_size;
                            },
                            1 => {
                                let v = Variable {
                                    name: s.to_string(),
                                    stack_pos: *offset,
                                    t: Type::Buf(buf_size)
                                };
                                map.insert(s, v);
                            },
                            ignore => {warning!("Arg ignored `{}`", ignore)},
                        }
                    },
                    Op::U32 => {
                        match i {
                            0 => {
                                *offset += 4;
                                let v = Variable {
                                    name: s.to_string(),
                                    stack_pos: *offset,
                                    t: Type::U32
                                };
                                map.insert(s, v);
                            },
                            1 => {
                                code!(ops, "    movl ${}, -{}(%rsp)\n", s, offset);
                            },
                            ignore => {warning!("Arg ignored `{}`", ignore)},
                        }
                    },
                    _ => {
                        if i == 0 {
                            code!(ops, "    movl ${}, %eax\n", s);
                        } else {
                            code!(ops, "    {} ${}, %eax\n", asm, s);
                        }
                    },
                }
            },
            Expression::List(li2) => {
                match op {
                    Op::Syscall => {
                        todo!()
                    },
                    _ => {
                        if i == 0 {
                            eval(ops, li2, offset);
                        } else {
                            code!(ops, "    pushq %rax\n");
                            eval(ops, li2, offset);
                            code!(ops, "    movl %eax, %ecx\n");
                            code!(ops, "    popq %rax\n");
                            code!(ops, "    {} %ecx, %eax\n", asm);
                        }
                    },
                }
            },
        };
    }
    match op {
        Op::Syscall => {
            code!(ops, "    syscall\n");
            code!(ops, "    retq\n");
        },
        _ => {},
    }

    for k in map.keys() {
        println!("KEY: {}", k);
    }
    for v in map.values() {
        println!("VAL: {:?}", v);
    }

    let ret = map.get("one");
    dbg!(ret);
}

pub fn encode(_cst: Vec<Expression>, _filename: String) {

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
    subq $0, %rsp

");
        buf.append(&mut d1);
    }

    let mut stack_offset: i32 = 0;

    // let mut cs = CompState {
        // ops: String::new(),
        // cst: _cst,
        // offset: 0,
        // state: HashMap::new()
    // }


    // eval(&mut cs);

    for item in _cst.into_iter() {
        match item {
            Expression::List(v) => {
                let mut ops = String::new();
                eval(&mut ops, v, &mut stack_offset);
                let mut vec = Vec::from(ops);
                // println!("{:?}", &vec);
                buf.append(&mut vec);
            },
            Expression::Atom(s) => {
                warning!("Top level Atom `{}` ignored.", s);
            },
        }
    }

    // {
        // let ops = String::from("745\n");
        // let a: Vec<u8> = Vec::from(ops);
        // println!("{:?}", &a);
        // let b: Vec<u8> = Vec::from([7, 4, 5]);
        // println!("{:?}", &b);
        // println!("754: {:b}", 745);
        // println!("7  : {:b}", 7);
        // println!(" 5 : {:b}", 4);
        // println!("  4: {:b}", 5);
    // }

    {
    let mut d1 = Vec::from("
    movq $60, %rax
    movq $0, %rdi
    syscall

    addq $0, %rsp
    popq %rbp
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

    {
        let content = String::from_utf8(buf).unwrap();
        system!("wrote `{}` as `{}`", full_name, content);
    }
}

