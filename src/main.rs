
mod cli;
mod caeser;

use clap::builder::Str;
use crate::cli::cli::parse_args;

fn main() {
    let file: String = parse_args();
    println!("The contents of the file are:\n{}", file);

    let c: String = caeser::caeser::enc_caeser(file, 3);
    println!("Ciphertext:\n{}", c);

    let m : String = caeser::caeser::dec_caeser(c, 3);
    print!("Plaintext:\n{}", m);
}

