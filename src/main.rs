use std::fs::File;
use std::io::{BufReader, Read};

mod lexer;
mod parser;

use parser::Parser;

use clap::Parser as ClapParser;
use ndl_parser::*;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // let _cfg = config::get_config().expect("Was not able to get config");
    let _cfg = get_config(&cli.config_path.unwrap_or_default()).unwrap_or_default();

    let mut p = Parser::new(
        cli.file_path.clone(),
        get_contents_via_mut_buffer(&cli.file_path),
    );

    let parsed_tokens = p.parse_lr1();

    println!("{:?}", parsed_tokens);

    Ok(())
}

fn get_contents_via_mut_buffer(file_path: &str) -> String {
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

    /// Config file, if `Config.ndl` does not exist
    #[arg(short, long)]
    config_path: Option<String>,

    /// The output file to use
    #[arg(short, default_value = "main")]
    output: Option<String>,
}
