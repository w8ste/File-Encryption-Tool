use clap::Parser;
use std::fs::File;
use std::io::{self, Read};
use clap::builder::Str;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File to read
    #[arg(short, long)]
    file: String,
}

pub fn parse_args() -> String{
    let args = Args::parse();

    println!("The file is called: {}!", args.file);
    let contents = match read_file(args.file)  {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("An error has occurred whilst accessing the file: {}!", e);
            std::process::exit(1);
        }
    };

    contents
}

fn read_file(file_name : String) -> io::Result<String> {
    let mut file : File = File::open(file_name)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

