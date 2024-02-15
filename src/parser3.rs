
mod lexer;
use lexer::{Token, Keyword};

pub fn parse(input: String) {
    let mut lexed = lexer::lex(input);

}
