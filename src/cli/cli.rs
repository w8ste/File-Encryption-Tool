use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File to read
    #[arg(short, long)]
    file: String,
}

pub fn parse_args() {
    let args = Args::parse();

    println!("The file is called: {}!", args.file)
}

