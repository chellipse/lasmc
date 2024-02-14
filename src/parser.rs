use std::collections::HashMap;
use lazy_static::lazy_static; // For static HashMap initialization

mod lexer;
use lexer::{Token, Keyword};

#[derive(Debug, Clone, PartialEq)]
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

    let mut result = Expr {
        kind: quoted.clone(),
        content: vec![],
    };

    let mut quote_macro = Kind::Code;
    let mut quote_word = Kind::Code;

    let len = tokens.len();
    let mut i = 0;
    while i < len {
        match &tokens[i] {
            Token::LeftParen => {
                i += 1;
                let (y, expr) = parse_expr(&tokens[i..], quote_macro.clone());
                i += y;
                result.content.push(Content::Expr(expr));
            },
            Token::RightParen => {
                i += 1;
                break
            },
            Token::Quote => {
                quote_macro = Kind::Data;
                i += 1;
                match &tokens[i] {
                    Token::LeftParen => {
                        let (y, expr) = parse_expr(&tokens[i..], quote_macro.clone());
                        i += y;
                        quote_macro = Kind::Code;
                        result.content.push(Content::Expr(expr));
                    },
                    Token::Atom(s) => {
                        let expr = Expr {
                            kind: quote_macro.clone(),
                            content: vec![Content::Atom(s.clone())],
                        };
                        i += 1;
                        result.content.push(Content::Expr(expr));
                    },
                    _ => {},
                }
            },
            Token::Atom(s) => {
                if quote_macro == Kind::Data {
                    let expr = Expr {
                        kind: quote_macro,
                        content: vec![Content::Atom(s.clone())],
                    };
                    result.content.push(Content::Expr(expr));
                    quote_macro = Kind::Code;
                } else {
                    result.content.push(Content::Atom(s.clone()));
                }
                i += 1;
            },
            Token::KW(kw) => {
                i += 1;
                if *kw == Keyword::Quote {
                    match &tokens[i] {
                        Token::LeftParen => {
                            let (y, expr) = parse_expr(&tokens[i..], Kind::Data);
                            i += y;
                            result = expr;
                        },
                        Token::Atom(s) => {
                            let expr = Expr {
                                kind: Kind::Data,
                                content: vec![Content::Atom(s.clone())],
                            };
                            i += 1;
                            result.content.push(Content::Expr(expr));
                        },
                        _ => {},
                    }
                } else {
                    result.content.push(Content::Keyword(*kw));
                }
            },
        }
    }

    (i, result)
}

pub fn parse(input: String) {
    let mut lexed = lexer::lex(input);

    // replace Atoms which are a keyword with the Keyword enum
    for token in lexed.iter_mut() {
        if let Token::Atom(s) = token {
            if let Some(kw) = STR_TO_KW.get(s.as_str()) {
                *token = Token::KW(*kw)
            }
        }
    }

    let mut result: Vec<Expr> = vec![];

    // let mut result = Expr {
        // kind: quoted.clone(),
        // content: vec![],
    // };

    let mut quote_macro = Kind::Code;
    let len = lexed.len();
    let mut i = 0;

    while i < len {
        match &lexed[i] {
            Token::LeftParen => {
                i += 1;
                let (y, expr) = parse_expr(&lexed[i..], quote_macro.clone());
                i += y;
                result.push(expr);
                quote_macro = Kind::Code;
            },
            Token::RightParen => {
                // i += 1;
                break;
            },
            Token::Quote => {
                quote_macro = Kind::Data;
                i += 1;
            },
            Token::Atom(a) => {
                // result.content.push(Content::Atom(s.clone()));
                if quote_macro == Kind::Data {
                    let expr = Expr {
                        kind: quote_macro,
                        content: vec![Content::Atom(a.clone())]
                    };
                    result.push(expr);
                    quote_macro = Kind::Code;
                }
                i += 1;
            },
            Token::KW(_) => {
                // result.content.push(Content::Keyword(*kw));
                i += 1;
            },
        }
    }
    dbg!(result);
}

