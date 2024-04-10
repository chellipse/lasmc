use std::fs::File;
use std::io::prelude::*;
use std::process::exit;
use std::collections::HashMap;

// local module
use crate::parser::{Expression, Op};

#[allow(unused_imports)]
use lasmc::{error, system, warning};

// #[allow(dead_code)]
// #[derive(Debug)]
// pub enum Op {
    // False,
    // Add,
    // Sub,
    // Mul,
    // Print,
    // Syscall,
    // Alloc,
    // U32,
// }

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

macro_rules! check_if_var {
    ($cs:ident, $arg:tt) => (
        match $cs.var_map.get(&$arg) {
            Some(var) => {
                format!("-{}(%rsp)", var.stack_pos)
            },
            None => format!("${}", $arg)
        }
    )
}

macro_rules! code {
    ($cs:ident, $fmt:expr) => (
        $cs.ops.push_str($fmt);
    );
    ($cs:ident, $fmt:expr, $($arg:tt)*) => (
        $cs.ops.push_str(format!($fmt, $($arg)*).as_str());
    )
}

#[derive(Debug)]
struct CompilationState {
    ops: String,
    offset: i32,
    var_map: HashMap<String, Variable>,
}

fn eval(state: &mut CompilationState, cst: Vec<Expression>) {
    // dbg!(&cst);
    let mut iter = cst.into_iter();

    let e1 = iter.next();
    let op: Op = match e1 {
        Some(Expression::Key(kw)) => {
            kw
        },
        _ => {todo!()},
    };
    // dbg!(&op);

    let asm = match op {
        Op::Add => {"addl"},
        Op::Sub => {"subl"},
        Op::Mul => {"imul"},
        ref _o  => {""},
    };
    // dbg!(&asm);

    let mut buf_size: i32 = 0;

    for (i, item) in iter.enumerate() {
        match item {
            Expression::List(li2) => {
                match op {
                    Op::Syscall => {
                        todo!()
                    },
                    Op::U32 => {
                        if i == 0 {
                            todo!()
                        } else {
                            eval(state, li2);
                            code!(state, "    movl %eax, -{}(%rsp)\n", state.offset);
                        }
                    },
                    _ => {
                        if i == 0 {
                            eval(state, li2);
                        } else {
                            // dbg!(&op);
                            code!(state, "    pushq %rax\n");
                            eval(state, li2);
                            code!(state, "    movl %eax, %ecx\n");
                            code!(state, "    popq %rax\n");
                            code!(state, "    {} %ecx, %eax\n", asm);
                        }
                    },
                }
            },
            Expression::Imm(s) => {
                match op {
                    Op::Syscall => {
                        match i {
                            0 => {
                                code!(state, "    movq {}, %rax\n", s);
                            },
                            1 => {
                                code!(state, "    movq {}, %rdi\n", s);
                            },
                            2 => {
                                code!(state, "    leaq {}, %rsi\n", s);
                            },
                            3 => {
                                code!(state, "    movq {}, %rdx\n", s);
                            },
                            ignore => {warning!("Arg ignored `{}`", ignore)},
                        }
                    },
                    Op::Alloc => {
                        match i {
                            0 => {
                                buf_size = s.parse::<i32>().unwrap();
                                state.offset += buf_size;
                            },
                            1 => {
                                error!("Alloc identifier must be a variable name: {:?}", s);
                            },
                            ignore => {warning!("Arg ignored `{}`", ignore)},
                        }
                    },
                    Op::U32 => {
                        match i {
                            0 => {
                                error!("U32 identifier must be a variable name: {:?}", s);
                            },
                            1 => {
                                code!(state, "    movl {}, -{}(%rsp)\n", s, state.offset);
                            },
                            ignore => {warning!("Arg ignored `{}`", ignore)},
                        }
                    },
                    _ => {
                        if i == 0 {
                            let v = check_if_var!(state, s);
                            code!(state, "    movl {}, %eax\n", v);
                        } else {
                            let v = check_if_var!(state, s);
                            code!(state, "    {} {}, %eax\n", asm, v);
                        }
                    },
                }
            },
            Expression::Var(v) => {
                match op {
                    Op::Syscall => {
                        match i {
                            0 => {
                                let v = check_if_var!(state, v);
                                code!(state, "    movq {}, %rax\n", v);
                            },
                            1 => {
                                let v = check_if_var!(state, v);
                                code!(state, "    movq {}, %rdi\n", v);
                            },
                            2 => {
                                let v = check_if_var!(state, v);
                                code!(state, "    leaq {}, %rsi\n", v);
                            },
                            3 => {
                                let v = check_if_var!(state, v);
                                code!(state, "    movq {}, %rdx\n", v);
                            },
                            ignore => {warning!("Arg ignored `{}`", ignore)},
                        }
                    },
                    Op::Alloc => {
                        match i {
                            0 => {
                                let v = check_if_var!(state, v);
                                buf_size = v.parse::<i32>().unwrap();
                                state.offset += buf_size;
                            },
                            1 => {
                                let var = Variable {
                                    name: v.to_string(),
                                    stack_pos: state.offset,
                                    t: Type::Buf(buf_size)
                                };
                                state.var_map.insert(v, var);
                            },
                            ignore => {warning!("Arg ignored `{}`", ignore)},
                        }
                    },
                    Op::U32 => {
                        match i {
                            0 => {
                                state.offset += 4;
                                let var = Variable {
                                    name: v.to_string(),
                                    stack_pos: state.offset,
                                    t: Type::U32
                                };
                                state.var_map.insert(v, var);
                            },
                            1 => {
                                let v = check_if_var!(state, v);
                                code!(state, "    movl {}, -{}(%rsp)\n", v, state.offset);
                            },
                            ignore => {warning!("Arg ignored `{}`", ignore)},
                        }
                    },
                    _ => {
                        if i == 0 {
                            code!(state, "    movl {}, %eax\n", v);
                        } else {
                            code!(state, "    {} {}, %eax\n", asm, v);
                        }
                    },
                }
            },
            Expression::Key(kw) => {
                error!("Unexpected keyword: `{:?}`", kw);
                std::process::exit(1);
            },
        };
    }
    match op {
        Op::Syscall => {
            code!(state, "    syscall\n");
            code!(state, "    retq\n");
        },
        _ => {},
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
    subq $0, %rsp

");
        buf.append(&mut d1);
    }

    let mut state = CompilationState {
        ops: String::new(),
        offset: 0,
        var_map: HashMap::new()
    };

    for item in cst.into_iter() {
        match item {
            Expression::List(v) => {
                eval(&mut state, v);
            },
            other => {
                warning!("Top level Atom `{:?}` ignored.", other);
            },
        }
    }
    let mut vec = Vec::from(state.ops);
    buf.append(&mut vec);

    // {
        // for k in state.var_map.keys() {
            // println!("KEY: {}", k);
        // }
        // for v in state.var_map.values() {
            // println!("VAL: {:?}", v);
        // }
        // let ret = state.var_map.get("one");
        // dbg!(ret);
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

