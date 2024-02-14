use regex::Regex;

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    Quote,
    Atom,
    Eq,
    Car,
    Cdr,
    Cons,
    Cond,
}

#[derive(Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    Quote,
    Atom(String),
    KW(Keyword),
}

pub fn lex(input: String) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];

    let re_leftparen = Regex::new(r"^\(").unwrap();
    let re_rightparen = Regex::new(r"^\)").unwrap();
    let re_quote = Regex::new(r"^'").unwrap();
    let re_atom = Regex::new(r"^[a-z]+").unwrap();
    let re_ignore = Regex::new(r"^( |\n)+").unwrap();

    let len = input.len();
    let mut i = 0;
    while i < len {
        match input {
            // (
            _ if re_leftparen.is_match(&input[i..]) => {
                // println!("LEFTPAREN");
                result.push(Token::LeftParen);
                i += 1;
            }
            // )
            _ if re_rightparen.is_match(&input[i..]) => {
                // println!("RIGHTPAREN");
                result.push(Token::RightParen);
                i += 1;
            }
            // '
            _ if re_quote.is_match(&input[i..]) => {
                // println!("QUOTE");
                result.push(Token::Quote);
                i += 1;
            }
            // Atom
            _ if re_atom.is_match(&input[i..]) => {
                let v = re_atom.find(&input[i..]).map(|s| s.as_str()).unwrap();
                result.push(Token::Atom(String::from(v)));
                i += v.len();
                // println!("ATOM: {:?}, LEN: {:?}", v, v.len());
            }
            // Ignore
            _ if re_ignore.is_match(&input[i..]) => {
                // println!("IGNORE");
                i += 1;
            }
            _ => {
                eprintln!("Invalid: at {} in {}", i, input);
                eprintln!("Pro: {:?}", result);
                break
            },
        }
    }
    result
}
