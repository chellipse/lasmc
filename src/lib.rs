macro_rules! check_if_var {
    ($cs:ident, $arg:tt) => {
        match $cs.var_map.get(&$arg) {
            Some(var) => {
                format!("-{}(%rsp)", var.stack_pos)
            }
            None => format!("${}", $arg),
        }
    };
}

use std::collections::HashMap;

#[derive(Debug)]
pub enum Op {
    False,
    Add,
    Sub,
    Mul,
    Print,
    Syscall,
    Alloc,
    U32,
}

#[derive(Debug)]
pub enum Expression {
    Key(Op),
    Imm(String),
    Var(String),
    List(Vec<Expression>),
}

#[derive(Debug)]
pub enum Type {
    Buf(i32),
    U32,
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub stack_pos: i32,
    pub t: Type,
}

#[derive(Debug)]
pub struct CompilationState {
    pub ops: String,
    pub offset: Vec<i32>,
    pub var_map: HashMap<String, Variable>,
}

impl CompilationState {
    pub fn off_inc(&mut self, i: i32) {
        let len = self.offset.len();
        let new_off = self.offset[len-1] + i;
        self.offset.push(new_off)
    }
    pub fn off_get(&self) -> i32 {
        self.offset[self.offset.len()-1]
    }
    pub fn off_last_inc(&self) -> i32 {
        let len = self.offset.len();
        let current = self.offset[len-1];
        let last = self.offset[len-2];
        current - last
    }
}

impl Op {
    pub fn encode_imm(&self, state: &mut CompilationState, pos: usize, token: String) {
        let assembly = match self {
            Op::Add => {
                if pos == 0 {
                    format!("    movl {}, %eax\n", token)
                } else {
                    format!("    addl {}, %eax\n", token)
                }
            }
            Op::Sub => {
                if pos == 0 {
                    format!("    movl {}, %eax\n", token)
                } else {
                    format!("    subl {}, %eax\n", token)
                }
            }
            Op::Mul => {
                if pos == 0 {
                    format!("    movl {}, %eax\n", token)
                } else {
                    format!("    imul {}, %eax\n", token)
                }
            }
            Op::Syscall => {
                match pos {
                    0 => {
                        format!("    movq {}, %rax\n", token)
                    }
                    1 => {
                        format!("    movq {}, %rdi\n", token)
                    }
                    2 => {
                        format!("    leaq {}, %rsi\n", token)
                    }
                    3 => {
                        format!("    movq {}, %rdx\n", token)
                    }
                    ignore => {
                        error!("Arg ignored `{}`", ignore);
                        exit(101)
                    }
                }
            }
            Op::U32     => {
                match pos {
                    0 => {
                        error!("U32 identifier must be a variable name: {:?}", token);
                        exit(101)
                    }
                    1 => {
                        format!("    movl {}, -{}(%rsp)\n", token, state.off_get())
                    }
                    ignore => {
                        error!("Arg ignored `{}`", ignore);
                        exit(101)
                    }
                }
            }
            Op::Alloc   => {
                match pos {
                    0 => {
                        state.off_inc(token.parse::<i32>().unwrap());
                        String::new()
                    }
                    1 => {
                        error!("Alloc identifier must be a variable name: {:?}", token);
                        exit(101)
                    }
                    ignore => {
                        warning!("Arg ignored `{}`", ignore);
                        exit(101)
                    }
                }
            }
            Op::False   => {
                match pos {
                    _ => todo!()
                }
            }
            Op::Print   => {
                match pos {
                    _ => todo!()
                }
            }
        };
        state.ops.push_str(assembly.as_str());
    }

    pub fn encode_var(&self, state: &mut CompilationState, pos: usize, token: String) {
        // MASSIVE VARIABLE SIZE ASSUMPTIONS!!!
        let assembly = match self {
            Op::Add => {
                if pos == 0 {
                    let token = check_if_var!(state, token);
                    format!("    movl {}, %eax\n", token)
                } else {
                    let token = check_if_var!(state, token);
                    format!("    addl {}, %eax\n", token)
                }
            }
            Op::Sub => {
                if pos == 0 {
                    let token = check_if_var!(state, token);
                    format!("    movl {}, %eax\n", token)
                } else {
                    let token = check_if_var!(state, token);
                    format!("    subl {}, %eax\n", token)
                }
            }
            Op::Mul => {
                if pos == 0 {
                    let token = check_if_var!(state, token);
                    format!("    movl {}, %eax\n", token)
                } else {
                    let token = check_if_var!(state, token);
                    format!("    imul {}, %eax\n", token)
                }
            }
            Op::Syscall => {
                // SIZE ASSUMPTION
                // `movq` is an 8 byte op code
                // we should add the ability to adapting to different sizes later
                match pos {
                    0 => {
                        let token = check_if_var!(state, token);
                        format!("    movq {}, %rax\n", token)
                    }
                    1 => {
                        let token = check_if_var!(state, token);
                        format!("    movq {}, %rdi\n", token)
                    }
                    2 => {
                        let token = check_if_var!(state, token);
                        format!("    leaq {}, %rsi\n", token)
                    }
                    3 => {
                        let token = check_if_var!(state, token);
                        format!("    movq {}, %rdx\n", token)
                    }
                    ignore => {
                        error!("Arg ignored `{}`", ignore);
                        exit(101)
                    }
                }
            }
            Op::U32     => {
                match pos {
                    0 => {
                        state.off_inc(4);
                        let var = Variable {
                            name: token.to_string(),
                            stack_pos: state.off_get(),
                            t: Type::U32,
                        };
                        state.var_map.insert(token, var);
                        String::new()
                    }
                    1 => {
                        let token = check_if_var!(state, token);
                        format!("    movl {}, -{}(%rsp)\n", token, state.off_get())
                    }
                    ignore => {
                        error!("Arg ignored `{}`", ignore);
                        exit(101)
                    }
                }
            }
            Op::Alloc   => {
                match pos {
                    0 => {
                        error!("Alloc size must be immediate value: {:?}", token);
                        exit(101)
                    }
                    1 => {
                        let var = Variable {
                            name: token.to_string(),
                            stack_pos: state.off_get(),
                            t: Type::Buf(state.off_last_inc()),
                        };
                        state.var_map.insert(token, var);
                        String::new()
                    }
                    ignore => {
                        warning!("Arg ignored `{}`", ignore);
                        exit(101)
                    }
                }
            }
            Op::False   => {
                match pos {
                    _ => todo!()
                }
            }
            Op::Print   => {
                match pos {
                    _ => todo!()
                }
            }
        };
        state.ops.push_str(assembly.as_str());
    }
}

#[derive(Debug)]
pub struct Args {
    pub file_path: String,
    pub file_name: String,
}

use std::env;
use std::process::exit;

pub fn get_args() -> Args {
    let args: Vec<String> = env::args().collect(); // get command line arguments

    if args.len() != 2 {
        println!("Incorrect number of args.");
        eprintln!("Usage: {} <filename>", args[0]);
        exit(1)
    }

    let filepath = &args[1]; // the last argument is the file name

    let name_ext: &str = filepath.split('/').last().unwrap();
    let name: &str = name_ext.split('.').next().unwrap();

    Args {
        file_path: filepath.to_string(),
        file_name: name.to_string(),
    }
}

// attempts to read the first arg as file to string
// will Panic! if the file doesn't exist or cannot be read
pub fn read_input_file(filepath: String) -> String {
    use std::fs::read_to_string;
    match read_to_string(&filepath) {
        Ok(content) => content,
        Err(e) => {
            error!("reading `{}` returned `{}`", filepath, e);
            filepath.clone()
        }
    }
}

pub const ERR: &str = "\x1b[1;38;5;1m";
pub const WARN: &str = "\x1b[1;38;5;3m";
pub const SYSTEM: &str = "\x1b[1;38;5;32m";
pub const RESET: &str = "\x1b[0m";

#[macro_export]
macro_rules! error {
    ($fmt:expr) => (
    eprintln!("{}error{} at `{}`[{}:{}]: {}", $crate::ERR, $crate::RESET, file!(), line!(), column!(), $fmt)
);
    ($fmt:expr, $($arg:tt)*) => (
    eprintln!("{}error{} at `{}`[{}:{}]: {}", $crate::ERR, $crate::RESET, file!(), line!(), column!(), format!($fmt, $($arg)*))
)
}

#[macro_export]
macro_rules! warning {
    ($fmt:expr) => (
    eprintln!("{}warning{} at `{}`[{}:{}]: {}", $crate::WARN, $crate::RESET, file!(), line!(), column!(), $fmt)
);
    ($fmt:expr, $($arg:tt)*) => (
    eprintln!("{}warning{} at `{}`[{}:{}]: {}", $crate::WARN, $crate::RESET, file!(), line!(), column!(), format!($fmt, $($arg)*))
)
}

#[macro_export]
macro_rules! system {
    ($fmt:expr) => (
    println!("{}system{} at `{}`[{}:{}]: {}", $crate::SYSTEM, $crate::RESET, file!(), line!(), column!(), $fmt)
);
    ($fmt:expr, $($arg:tt)*) => (
    println!("{}system{} at `{}`[{}:{}]: {}", $crate::SYSTEM, $crate::RESET, file!(), line!(), column!(), format!($fmt, $($arg)*))
)
}
