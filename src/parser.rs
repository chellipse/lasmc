mod lexer;

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

pub fn parse(input: String) {
    let lexed = lexer::lex(input);
    println!("{lexed}");
}

