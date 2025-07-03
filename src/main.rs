mod absttext;
mod markdown;

use std::fs;
use std::error::Error;

use absttext::MarkupGenerator;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    input_path: String
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let absttext_input: String = fs::read_to_string(args.input_path)?;

    let output = markdown::MarkdownGenerator::generate(absttext::types::Essay::from_paragraph_string(absttext_input).unwrap());
    println!("{}", output);

    Ok(())
}
