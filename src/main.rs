mod parser;
mod file_reader;
mod encoder;

use asm_lisp::{get_args, read_input_file};

fn main() {
    let args = get_args();
    // dbg!(&args);

    let input = read_input_file(args.file_path);
    let cst = parser::parse(input);
    // dbg!(&cst);

    encoder::encode(cst, args.file_name);
}

