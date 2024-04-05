use lasmc::warning;

const ACCEPTED_CHARS: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    '1', '2', '3', '4', '5', '6', '7', '8', '9',
    '+', '-', '*',
];
const SILENT_CHARS: [char; 2] = [
    ' ', '\n'
];
const CUT_OFF_CHARS: [char; 3] = ['(', ')', ' '];

#[derive(Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    Atom(String),
}

pub fn lex(input: String) -> Vec<Token> {
    let chars: Vec<char> = input.chars().chain(std::iter::once(' ')).collect();
    // dbg!(&chars);

    let mut output: Vec<Token> = vec![];

    let len = chars.len();
    let mut i = 0;
    while i < len {
        match chars[i] {
            '(' => {
                output.push(Token::LeftParen);
                i += 1;
            },
            ')' => {
                output.push(Token::RightParen);
                i += 1;
            },
            _ if ACCEPTED_CHARS.contains(&chars[i]) => {
                let mut l = i.clone();
                while !CUT_OFF_CHARS.contains(&chars[l]) {
                    l += 1;
                }
                output.push(Token::Atom(chars[i..l].iter().collect()));
                i = l;
            },
            _ if SILENT_CHARS.contains(&chars[i]) => {
                i+=1;
            },
            ignored_char => {
                warning!("Unknown char ignored! `{}`", ignored_char);
                i+=1;
            },
        }
    }
    // dbg!(&output);
    output
}
