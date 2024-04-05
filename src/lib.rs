pub const ERR: &str = "\x1b[1;38;5;1m";
pub const WARN: &str = "\x1b[1;38;5;3m";
pub const SYSTEM: &str = "\x1b[1;38;5;32m";
pub const RESET: &str = "\x1b[0m";

#[macro_export]
macro_rules! error {
    ($fmt:expr) => (
    eprintln!("{}error{} at `{}`[{}:{}]: {}", $crate::ERR, $crate::RESET, file!(), line!(), column!(), $fmt)
);
    ($fmt:expr, $($arg:tt)*) => (
    eprintln!("{}error{} at `{}`[{}:{}]: {}", $crate::ERR, $crate::RESET, file!(), line!(), column!(), format!($fmt, $($arg)*))
)
}

#[macro_export]
macro_rules! warning {
    ($fmt:expr) => (
    eprintln!("{}warning{} at `{}`[{}:{}]: {}", $crate::WARN, $crate::RESET, file!(), line!(), column!(), $fmt)
);
    ($fmt:expr, $($arg:tt)*) => (
    eprintln!("{}warning{} at `{}`[{}:{}]: {}", $crate::WARN, $crate::RESET, file!(), line!(), column!(), format!($fmt, $($arg)*))
)
}

#[macro_export]
macro_rules! system {
    ($fmt:expr) => (
    println!("{}system{} at `{}`[{}:{}]: {}", $crate::SYSTEM, $crate::RESET, file!(), line!(), column!(), $fmt)
);
    ($fmt:expr, $($arg:tt)*) => (
    println!("{}system{} at `{}`[{}:{}]: {}", $crate::SYSTEM, $crate::RESET, file!(), line!(), column!(), format!($fmt, $($arg)*))
)
}

use std::fs::read_to_string;

// attempts to read the first arg as file to string
// will Panic! if the file doesn't exist or cannot be read
pub fn read_input_file(filepath: String) -> String {
    match read_to_string(&filepath) {
        Ok(content) => content,
        Err(e) => {
            error!("reading `{}` returned `{}`", filepath, e);
            filepath.clone()
        },
    }
}

#[derive(Debug)]
pub struct Args {
    pub file_path: String,
    pub file_name: String,
}

use std::env;
use std::process::exit;

pub fn get_args() -> Args {
    let args: Vec<String> = env::args().collect(); // get command line arguments

    if args.len() != 2 {
        println!("Incorrect number of args.");
        eprintln!("Usage: {} <filename>", args[0]);
        exit(1)
    }

    let filepath = &args[1]; // the last argument is the file name

    let name_ext: &str = filepath.split('/').last().unwrap();
    let name: &str = name_ext.split('.').next().unwrap();

    Args {
        file_path: filepath.to_string(),
        file_name: name.to_string(),
    }
}

