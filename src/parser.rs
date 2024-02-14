use std::collections::HashMap;
use lazy_static::lazy_static; // For static HashMap initialization

mod lexer;
use lexer::{Token, Keyword};

enum Kind {
    Code,
    Data,
}

enum Content {
    Keyword(Keyword),
    Atom(String),
    Expr(Expr),
}

struct Expr {
    kind: Kind,
    content: Vec<Content>,
}

lazy_static! {
    static ref STR_TO_KW: HashMap<&'static str, Keyword> = {
        let mut m = HashMap::new();
        m.insert("quote", Keyword::Quote);
        m.insert("atom", Keyword::Atom);
        m.insert("eq", Keyword::Eq);
        m.insert("car", Keyword::Car);
        m.insert("cdr", Keyword::Cdr);
        m.insert("cons", Keyword::Cons);
        m.insert("cond", Keyword::Cond);
        m
    };
}

fn parse_expr(tokens: &[Token]) -> (usize, Expr) {
    println!("{:?}", tokens);

    let mut result = Expr {
        kind: Kind::Code,
        content: vec![],
    };

    let len = tokens.len();
    let mut i = 0;
    while i < len {
        match &tokens[i] {
            // _ => println!("{:?}", &tokens[i]),
            Token::LeftParen => {
                let (y, expr) = parse_expr(&tokens[i..]);
                i += y;
                result.content.push(Content::Expr(expr));
            },
            Token::RightParen => {
                break
            },
            Token::Quote => result.kind = Kind::Data,
            Token::Atom(s) => {},
            Token::KW(kw) => {},
        }
    }

    let i = 0;
    let expr = Expr {
        kind: Kind::Code,
        content: vec![],
    };
    (i, expr)
}

pub fn parse(input: String) {
    let mut lexed = lexer::lex(input);
    // println!("{:?}", lexed);

    for token in lexed.iter_mut() {
        if let Token::Atom(s) = token {
            if let Some(kw) = STR_TO_KW.get(s.as_str()) {
                *token = Token::KW(*kw)
            }
        }
    }
    println!("{:?}", lexed);

    let mut result: Vec<Expr> = vec![];

    let len = lexed.len();
    let mut i = 0;
    while i < len {
        let (c, expr) = parse_expr(&lexed[i..]);
    }
}

