mod parser;
mod encoder;

#[allow(unused_imports)]
use lasmc::{error, system, warning};

use lasmc::{get_args, read_input_file};

fn main() {
    let args = get_args();
    // dbg!(&args);

    let input = read_input_file(args.file_path);
    dbg!(&input);
    let cst = parser::parse(input);
    // dbg!(&cst);

    encoder::encode(cst, args.file_name);
}

