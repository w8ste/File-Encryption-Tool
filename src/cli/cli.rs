use clap::Parser;
use std::fs::File;
use std::io::{self, Read, Write};
use std::fs;
use clap::builder::Str;

use crate::caeser::caeser::enc_caeser;

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


pub fn welcome() {
    const WELCOME_MESSAGE: &str = r#"
Welcome to Crypt Hippo!"#;

    println!("{}", WELCOME_MESSAGE);
    event_loop()
}

fn event_loop() {
    const MENU : &str = r#"
Choose one of the following:
1. Encrypt a file
2. Decrypt a file
3. Exit

Enter the number corresponding to your choice:"#;
    loop {
        println!("{}", MENU);
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(option) => {
                match option {
                    1 => encrypt_file(),
                    2 => println!("dec"),
                    3 => {
                        println!("See you soon!");
                        std::process::exit(1);
                    }
                    _ => println!("Invalid option, please try again!")
                }
            }
            Err(e) => println!("Invalid option, please try again")
        }

    }
}

fn encrypt_file() {
    const START : &str = r#"You have choosen to encrypt a file!
Please specify the path to your file:"#;
    const ENCMENU : &str = r#"
Choose one of the following:
1. CAESER
2. GO BACK

Enter the number corresponding to your choice:"#;


    println!("{}", START);
    print!("> ");
    io::stdout().flush().unwrap();

    let mut file_path = String::new();

    io::stdin().read_line(&mut file_path).unwrap();

    println!("{}", ENCMENU);
    print!("> ");
    io::stdout().flush().unwrap();

    let mut enc = String::new();

    io::stdin().read_line(&mut enc).unwrap();

    match enc.trim().parse::<u32>() {
        Ok(option) => {
            match option {
                1 => caeser(file_path.trim().parse().unwrap()),
                2 => return,
                _ => println!("Invalid option, please try again!")
            }
        }
        Err(e) => println!("Invalid option, please try again")
    }

}

fn caeser(file_path : String) {

    println!("The file is called: {}!", file_path);
    let contents: String = match read_file(file_path)  {
        Ok(contents) => contents,
        Err(e) => {
            // print error message and exit from the program
            eprintln!("An error has occurred whilst accessing the file: {}!", e);
            std::process::exit(1);
        }
    };

    println!("The contents of your file are: {}", contents);

    // 3 is the default value for caeser cypher
    let cipher = enc_caeser(&contents, 3);

    println!("The cipher is: {}\n", cipher);

}


