
mod lexer;
use lexer::Token;

use lasmc::{error, warning};

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Debug)]
pub enum Expression {
    Key(Op),
    Imm(String),
    Var(String),
    List(Vec<Expression>),
}

fn parse_vec(input: &[Token]) -> (usize, Expression) {
    let mut collector: Vec<Expression> = vec![];

    let len = input.len();
    let mut i = 0;
    while i < len {
        match &input[i] {
            Token::LeftParen => {
                i += 1;
                let (l, recursive_output) = parse_vec(&input[i..]);
                i += l;
                collector.push(recursive_output);
            },
            Token::RightParen => {
                i += 1;
                break
            },
            Token::Atom(s) => {
                i += 1;
                collector.push(Expression::Imm(s.clone()));
            },
        }
    }
    let output: Expression = Expression::List(collector);
    (i, output)
}

fn tree_pass(cst: &mut Vec<Expression>) {
    for node in cst.iter_mut() {
        match node {
            Expression::Imm(s) => {
                match s.as_str() {
                    "+" => { *node = Expression::Key(Op::Add); },
                    "-" => { *node = Expression::Key(Op::Sub); },
                    "*" => { *node = Expression::Key(Op::Mul); },
                    "syscall" => { *node = Expression::Key(Op::Syscall); },
                    "alloc" => { *node = Expression::Key(Op::Alloc); },
                    "u32" => { *node = Expression::Key(Op::U32); },
                    x if x.parse::<u32>().is_ok() => {
                        *node = Expression::Imm(s.clone());
                    },
                    _ => {
                        *node = Expression::Var(s.clone());
                    },
                }
            },
            Expression::List(li) => {
                tree_pass(li);
            },
            _ => {},
        }
    }
}


pub fn parse(input: String) -> Vec<Expression> {
    let lexed = lexer::lex(input);

    let mut output: Vec<Expression> = Vec::new();

    let len = lexed.len();
    let mut i = 0;
    while i < len {
        match &lexed[i] {
            Token::LeftParen => {
                i += 1;
                let (l, recursive_output) = parse_vec(&lexed[i..]);
                i += l;
                output.push(recursive_output);
            },
            Token::RightParen => {
                error!("Unmatched ')' at {}", i);
            },
            Token::Atom(_) => {
                warning!("Ignored Atom at {}", i);
                i += 1;
            },
        }
    }
    tree_pass(&mut output);
    output
}

