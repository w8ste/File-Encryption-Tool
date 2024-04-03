use clap::Parser;
use std::fs::File;
use std::io::{self, Read};
use std::fs;
use clap::builder::Str;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File to read
    #[arg(short, long)]
    file: String,

    /// Determine if the file will be encrypted
    #[arg(short, long, default_value_t = 1)]
    encrypt: u8,
}

pub fn parse_args() -> String{
    let args : Args = Args::parse();

    println!("The file is called: {}!", args.file);
    let contents: String = match read_file(args.file)  {
        Ok(contents) => contents,
        Err(e) => {
            // print error message and exit from the program
            eprintln!("An error has occurred whilst accessing the file: {}!", e);
            std::process::exit(1);
        }
    };

    contents
}

/*
* file_name: path/filename
* This method will attempt to read the contents of the specified file and returns
* contents or propagate an error
 */
fn read_file(file_name : String) -> io::Result<String> {
    let mut file : File = File::open(file_name)?;

    let mut contents : String = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn write_file(content : String, path : String) -> io::Result<()>{
    fs::write(path, content)?;

    Ok(())
}

