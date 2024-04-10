// use crate::parser::{Expression, Op};

mod encoder;
mod parser;

#[allow(unused_imports)]
use lasmc::{error, system, warning};

use lasmc::{get_args, read_input_file};

// fn tree_pass(cst: &mut Vec<Expression>) {
// for node in cst.iter_mut() {
// match node {
// Expression::Imm(s) => {
// match s.as_str() {
// "+" => { *node = Expression::Key(Op::Add); },
// "-" => { *node = Expression::Key(Op::Sub); },
// "*" => { *node = Expression::Key(Op::Mul); },
// "syscall" => { *node = Expression::Key(Op::Syscall); },
// "alloc" => { *node = Expression::Key(Op::Alloc); },
// "u32" => { *node = Expression::Key(Op::U32); },
// x if x.parse::<u32>().is_ok() => {
// *node = Expression::Imm(s.clone());
// },
// _ => {
// *node = Expression::Var(s.clone());
// },
// }
// },
// Expression::List(li) => {
// tree_pass(li);
// },
// _ => {},
// }
// }
// }

fn main() {
    let args = get_args();
    // dbg!(&args);

    let input = read_input_file(args.file_path);
    dbg!(&input);
    let cst = parser::parse(input);
    // tree_pass(&mut cst);
    // dbg!(&cst);

    // let x = "12";
    // let b = x.parse::<u32>().is_ok();

    encoder::encode(cst, args.file_name);
}
