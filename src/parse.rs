use regex::Regex;

enum Kind {
    Code,
    Data,
}

enum Content {
    Expr(Expr),
    Atom(String),
}

struct Expr {
    kind: Kind,
    content: Vec<Content>,
}

enum Token {
    LeftParen,
    RightParen,
    Item(String),
}

fn lex(input: String) {
    let result: Vec<&str> = input.split_whitespace().collect();

    let re_leftparen = Regex::new(r"^\(").unwrap();
    let re_rightparen = Regex::new(r"^\)").unwrap();
    let re_atom = Regex::new(r"^[a-z]+").unwrap();

    let i = 0;
    let len = input.len() - 1;
    while i < len {

    }

    println!("{:?}", result)
}

pub fn parse(input: String) {
    lex(input);
}

