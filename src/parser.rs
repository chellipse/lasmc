use std::collections::HashMap;
use lazy_static::lazy_static; // For static HashMap initialization

mod lexer;
use lexer::{Token, Keyword};

#[derive(Debug, Clone)]
enum Kind {
    Code,
    Data,
}

#[derive(Debug)]
enum Content {
    Keyword(Keyword),
    Atom(String),
    Expr(Expr),
}

#[derive(Debug)]
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

fn parse_expr(tokens: &[Token], quoted: Kind) -> (usize, Expr) {
    // println!("{:?}", tokens);

    let mut result = Expr {
        kind: quoted.clone(),
        content: vec![],
    };

    let mut quote_tracker = Kind::Code;

    let len = tokens.len();
    let mut i = 0;
    while i < len {
        println!("{:?}", &tokens[i]);
        match &tokens[i] {
            // _ => println!("{:?}", &tokens[i]),
            Token::LeftParen => {
                i += 1;
                let (y, expr) = parse_expr(&tokens[i..], quote_tracker.clone());
                i += y;
                result.content.push(Content::Expr(expr));
            },
            Token::RightParen => {
                i += 1;
                break
            },
            Token::Quote => {
                quote_tracker = Kind::Data;
                i += 1;
            },
            Token::Atom(s) => {
                result.content.push(Content::Atom(s.clone()));
                i += 1;
            },
            Token::KW(kw) => {
                result.content.push(Content::Keyword(*kw));
                i += 1;
            },
        }
    }

    (i, result)
}

pub fn parse(input: String) {
    let mut lexed = lexer::lex(input);
    // println!("{:?}", lexed);

    // replace Atoms which are a keyword with the Keyword enum
    for token in lexed.iter_mut() {
        if let Token::Atom(s) = token {
            if let Some(kw) = STR_TO_KW.get(s.as_str()) {
                *token = Token::KW(*kw)
            }
        }
    }
    println!("{:?}", lexed);

    let mut result: Vec<Expr> = vec![];

    // let mut result = Expr {
        // kind: quoted.clone(),
        // content: vec![],
    // };

    let mut quote_tracker = Kind::Code;
    let len = lexed.len();
    let mut i = 0;

    while i < len {
        println!("{:?}", &lexed[i]);
        match &lexed[i] {
            // _ => println!("{:?}", &lexed[i]),
            Token::LeftParen => {
                i += 1;
                let (y, expr) = parse_expr(&lexed[i..], quote_tracker.clone());
                i += y;
                result.push(expr);
            },
            Token::RightParen => {
                // i += 1;
                break;
            },
            Token::Quote => {
                quote_tracker = Kind::Data;
                i += 1;
            },
            Token::Atom(_) => {
                // result.content.push(Content::Atom(s.clone()));
                i += 1;
            },
            Token::KW(_) => {
                // result.content.push(Content::Keyword(*kw));
                i += 1;
            },
        }
    }
    println!("RES: {:#?}", result);
}

