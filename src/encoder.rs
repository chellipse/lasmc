use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[allow(unused_imports)]
use crate::{
    error,
    warning,
    system,
};

use lasmc::{
    Expression,
    Op,
    CompilationState,
};

macro_rules! code {
    ($cs:ident, $fmt:expr) => (
        $cs.ops.push_str($fmt);
    );
    ($cs:ident, $fmt:expr, $($arg:tt)*) => (
        $cs.ops.push_str(format!($fmt, $($arg)*).as_str());
    )
}

fn eval(state: &mut CompilationState, cst: Vec<Expression>) {
    // dbg!(&cst);
    let mut iter = cst.into_iter();

    let e1 = iter.next();
    let op: Op = match e1 {
        Some(Expression::Key(kw)) => kw,
        _ => {
            todo!()
        }
    };

    let asm = match op {
        Op::Add => "addl",
        Op::Sub => "subl",
        Op::Mul => "imul",
        ref _o => "",
    };

    for (i, item) in iter.enumerate() {
        match item {
            Expression::List(li2) => {
                match op {
                    Op::Syscall => {
                        todo!()
                    }
                    Op::U32 => {
                        if i == 0 {
                            todo!()
                        } else {
                            eval(state, li2);
                            code!(state, "    movl %eax, -{}(%rsp)\n", state.off_get());
                        }
                    }
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
                    }
                }
            }
            Expression::Imm(s) => {
                op.encode_imm(state, i, s);
            },
            Expression::Var(v) => {
                op.encode_var(state, i, v);
            },
            Expression::Key(kw) => {
                error!("Unexpected keyword: `{:?}`", kw);
                std::process::exit(1);
            }
        };
    }
    match op {
        Op::Syscall => {
            code!(state, "    syscall\n");
            code!(state, "    retq\n");
        }
        _ => {}
    }
}

pub fn encode(cst: Vec<Expression>, _filename: String) {
    let full_name = format!("ignore/asm.s");
    let mut file = match File::create(&full_name) {
        Ok(f) => f,
        Err(e) => {
            error!("creating {} resulted in {}", full_name, e);
            std::process::exit(1);
        }
    };

    let mut buf: Vec<u8> = Vec::new();
    {
        let mut d1 = Vec::from(
            ".global _start

.text

_start:
    pushq %rbp
    movq %rsp, %rbp
    subq $0, %rsp

",
        );
        buf.append(&mut d1);
    }

    let mut state = CompilationState {
        ops: String::new(),
        offset: vec!(0),
        var_map: HashMap::new(),
    };

    for item in cst.into_iter() {
        match item {
            Expression::List(v) => {
                eval(&mut state, v);
            }
            other => {
                warning!("Top level Atom `{:?}` ignored.", other);
            }
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
        let mut d1 = Vec::from(
            "
    movq $60, %rax
    movq $0, %rdi
    syscall

    addq $0, %rsp
    popq %rbp
    retq
",
        );
        buf.append(&mut d1);
    }

    match file.write_all(&buf) {
        Ok(_) => {}
        Err(e) => {
            error!("writing to {} returned {}", full_name, e);
        }
    };

    {
        let content = String::from_utf8(buf).unwrap();
        system!("wrote `{}` as `{}`", full_name, content);
    }
}
