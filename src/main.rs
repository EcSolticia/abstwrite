mod absttext;

use std::fs;
use std::error::Error;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    input_path: String
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let absttext_input: String = fs::read_to_string(args.input_path)?;

    println!("{}", absttext::matcher::matchers::match_paragraph(&absttext_input).unwrap());

    Ok(())
}
