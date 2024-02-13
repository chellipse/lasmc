use std::env;
use std::fs::read_to_string;
use std::process::exit;

// attempts to read the first arg as file to string
// will Panic! if the file doesn't exist or cannot be read
pub fn read_input_file() -> String {
    let args: Vec<String> = env::args().collect(); // get command line arguments

    if args.len() != 2 {
        println!("Incorrect number of args.");
        eprintln!("Usage: {} <filename>", args[0]);
        exit(1)
    }

    let filepath = &args[1]; // the last argument is the file name

    match read_to_string(filepath) {
        Ok(content) => content,
        Err(e) => panic!("Failed to read file, err: {}", e),
    }
}
