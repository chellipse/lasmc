use std::collections::HashMap;
use lazy_static::lazy_static; // For static HashMap initialization

mod lexer;
use lexer::{Token, Keyword};

#[derive(Debug, Clone, PartialEq)]
enum Kind {
    Code,
    List,
}

#[derive(Debug)]
enum Content {
    KW(Keyword),
    Atom(String),
    Expr(Expression),
}

#[derive(Debug)]
struct Expression {
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

fn quote(input: &[Token]) -> Result<(usize, Kind, Vec<Content>), usize> {
    // let len = input.len();
    let mut i = 1;
    match &input[i] {
        Token::LeftParen => {
            let mut pct = 1;
            let mut last = 1;
            for item in input[i+1..].iter() {
                match item {
                    Token::LeftParen => {
                        pct += 1;
                        last += 1;
                    },
                    Token::RightParen => {
                        pct -= 1;
                        last += 1;
                    },
                    _ => {
                        last += 1;
                    },
                };
                if pct < 1 { break };
            };
            // dbg!(&input[i+1..last]);
            let mut content: Vec<Content> = vec![];
            for item in &input[i+1..last] {
                let t = match item {
                    Token::Atom(s) => {
                        Content::Atom(s.clone())
                    },
                    _ => {
                        Content::Atom(String::from("nil"))
                    },
                };
                content.push(t)
            }
            Ok((i+1, Kind::List, content))
        },
        Token::Atom(a) => {
            Ok((i+1, Kind::List, vec![Content::Atom(a.clone())]))
        },
        Token::KW(a) => {
            Ok((i+1, Kind::List, vec![Content::KW(a.clone())]))
        },
        _ => {
            println!("Err: Unquotable token!");
            Err(i)
        },
    }
}

fn core(lexed: &[Token]) -> (usize, Expression) {
    dbg!(lexed);

    // set word to equal our operator
    let mut word: Keyword;
    match lexed[0] {
        Token::KW(kw) => {
            word = kw;
        },
        _ => {
            panic!("Not a keyword!!!")
        },
    }

    let mut i: usize = 0;
    let mut output: Expression;

    match word {
        Keyword::Quote => {
            let result = quote(&lexed[0..]);
            match result {
                Ok((l, k, c)) => {
                    let expression = Expression {
                        kind: k,
                        content: c,
                    };
                    i += l+1;
                    output = expression;
                },
                Err(l) => {
                    panic!("Err: Cannot quote!!!")
                },
            }
        },
        Keyword::Atom => {
            todo!()
        },
        Keyword::Eq => {
            todo!()
        },
        Keyword::Car => {
            todo!()
        },
        Keyword::Cdr => {
            todo!()
        },
        Keyword::Cons => {
            todo!()
        },
        Keyword::Cond => {
            todo!()
        },
    }
    (i, output)
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

    let mut output: Vec<Expression> = vec![];

    let len = lexed.len();
    let mut i = 0;

    while i < len {
        match &lexed[i] {
            Token::LeftParen => {
                println!("T:(");
                i += 1;
                let (l, result) = core(&lexed[i..]);
                i+=l;
                output.push(result);
            },
            Token::RightParen => {
                panic!("T:) at {}", i);
                // i += 1;
                break;
            },
            Token::Quote => {
                println!("T:QUOTE");
                let result = quote(&lexed[i..]);
                match result {
                    Ok((l, k, c)) => {
                        i += l;
                        dbg!(i);
                        dbg!(l);
                        let expression = Expression {
                            kind: k,
                            content: c,
                        };
                        output.push(expression);
                    },
                    Err(l) => { i += l; },
                }
            },
            Token::Atom(_) => {
                panic!("T:ATOM at {}", i);
                i += 1;
            },
            Token::KW(_) => {
                panic!("T:KW at {}", i);
                i += 1;
            },
        }
        dbg!(&output);
    }

    dbg!(lexed);
    dbg!(output);
}

