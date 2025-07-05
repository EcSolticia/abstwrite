mod absttext;

use std::fs;
use std::io::Error;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    input_path: String
}

fn main() {
    /*let args = Cli::parse();

    let absttext_input: Result<String, Error> = fs::read_to_string(args.input_path);

    if absttext_input.is_err() {
        println!("Could not read file.");
    } else {
        println!("{}", absttext_input.unwrap());
    }*/
}
