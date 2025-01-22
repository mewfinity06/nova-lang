use std::fs::File;
use std::io::{BufReader, Read};

use clap::Parser;
use lexer::Lexer;

mod lexer;

fn get_contents_via_mut_buffer(file_path: &String) -> String {
    let mut data = String::new();
    let f = File::open(file_path).expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    data
}

fn main() {
    let cmd = Command::parse();

    let mut lexer = Lexer::new(cmd.file_path.to_string(), get_contents_via_mut_buffer(&cmd.file_path));
    let tokens = lexer.lex();

    for token in tokens {
        println!("{}", token);
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = "Welcome to the Nova Language Compiler")]
struct Command {
    /// The input file to use
    file_path: String,

    /// The output file to use
    #[arg(short, default_value = "main")]
    output: Option<String>,

    /// Enable debug mode | To be deprecated
    #[arg(short, long)]
    debug: bool,

    /// Enable developer mode | To be deprecated
    #[arg(long)]
    dev: bool,
}
