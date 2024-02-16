
mod lexer;
use lexer::Token;

#[derive(Debug)]
pub enum Expression {
    Atom(String),
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
                collector.push(Expression::Atom(s.clone()));
            },
        }
    }
    let output: Expression = Expression::List(collector);
    (i, output)
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
                panic!("Unmatched ')' at {}", i);
            },
            Token::Atom(_) => {
                eprintln!("Ignored Atom at {}", i);
                i += 1;
            },
        }
    }
    output
}

