use crate::parser::Expression;

mod parser;
mod encoder;

#[allow(unused_imports)]
use lasmc::{error, system, warning};

use lasmc::{get_args, read_input_file};

#[allow(dead_code)]
#[derive(Debug)]
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
enum ExpressionTest {
    Key(Op),
    Imm(String),
    Var(String),
    List(Vec<ExpressionTest>),
}

fn tree_pass(cst: &Vec<Expression>) -> Vec<ExpressionTest> {
    let mut vec: Vec<ExpressionTest> = Vec::new();
    for item in cst.iter() {
        match item {
            Expression::Atom(s) => {
                match s.as_str() {
                    "+" => { vec.push(ExpressionTest::Key(Op::Add)); },
                    "-" => { vec.push(ExpressionTest::Key(Op::Sub)); },
                    "*" => { vec.push(ExpressionTest::Key(Op::Mul)); },
                    "syscall" => { vec.push(ExpressionTest::Key(Op::Syscall)); },
                    "alloc" => { vec.push(ExpressionTest::Key(Op::Alloc)); },
                    "u32" => { vec.push(ExpressionTest::Key(Op::U32)); },
                    x if x.parse::<u32>().is_ok() => {
                        vec.push(ExpressionTest::Imm(s.clone()));
                    },
                    _ => {
                        vec.push(ExpressionTest::Var(s.clone()));
                    },
                }
            },
            // Expression::Var(s) => {},
            Expression::List(li) => {
                vec.push(ExpressionTest::List(tree_pass(li)));
            },
        }
    }
    vec
}

fn main() {
    let args = get_args();
    // dbg!(&args);

    let input = read_input_file(args.file_path);
    dbg!(&input);
    let mut cst = parser::parse(input);
    let processed_cst = tree_pass(&cst);
    dbg!(&processed_cst);

    // let x = "12";
    // let b = x.parse::<u32>().is_ok();

    // encoder::encode(cst, args.file_name);
}

