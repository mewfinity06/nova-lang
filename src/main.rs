use std::fs::File;
use std::io::{BufReader, Read};

mod config;
mod lexer;
mod parser;

use clap::Parser as ClapParser;
use parser::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let _cfg = config::get_config().expect("Was not able to get config");

    let parser = Parser::new(
        cli.file_path.to_string(),
        get_contents_via_mut_buffer(&cli.file_path),
    );

    parser.parse_lr1();

    Ok(())
}

fn get_contents_via_mut_buffer(file_path: &String) -> String {
    let mut data = String::new();
    let f = File::open(file_path).expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    data
}

#[derive(ClapParser, Debug)]
#[command(version, about, long_about = "Welcome to the Nova Language Compiler")]
struct Cli {
    /// The input file to use
    file_path: String,

    /// The output file to use
    #[arg(short, default_value = "main")]
    output: Option<String>,
}
