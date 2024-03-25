mod parser;
mod file_reader;
mod encoder;

fn main() {

    let input = file_reader::read_input_file();
    let cst = parser::parse(input);
    // dbg!(&cst);

    let (_src, program) = encoder::encode(cst);

    program();
}

