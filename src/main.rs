
mod cli;
use crate::cli::cli::parse_args;

fn main() {
    let file: String = parse_args();
    print!("The contents of the file are:\n{}", file)
}

