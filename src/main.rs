use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::process::Command as RustCommand;

mod lexer;
mod config;
use clap::Parser;

use lexer::Lexer;
use config::Config;

fn main() -> anyhow::Result<()> {
    ///////////// Build NDL Parser ///////////////////////

    // May not work on other operating systems from Linux
    let dir_path = "ndl-parser"; // The directory that ndl-parser should live in
    let exe_name = "ndl-parser"; // Linux executables usually don’t have ".exe"

    let exe_path = Path::new(dir_path).join(exe_name);

    if !directory_exists(dir_path) {
        get_ndl_parser_from_github()?;
    }

    // If ndl-parser does not exits
    if !exe_path.exists() && !exe_path.is_file() {
        build_ndl_parser()?;
    }

    ///////////// Use NDL Parser ///////////////////////
    
    let ndl_parser = RustCommand::new(exe_path)
        .arg(".")
        .output()?;

    let ndl_parsed = format!("{}", String::from_utf8_lossy(&ndl_parser.stdout));

    println!("Parsed: {ndl_parsed}");

    let def: Config = serde_json::from_str(&ndl_parsed)?;

    println!("{:?}", def);

    Ok(())
}

fn get_ndl_parser_from_github() -> anyhow::Result<()> {
    RustCommand::new("sh")
        .arg("-c")
        .arg("git clone https://github.com/mewfinity06/ndl-parser.git")
        .output()?;

    println!("Built ndl-parser dir");

    Ok(())
}

fn build_ndl_parser() -> anyhow::Result<()> {
    RustCommand::new("sh")
        .arg("-c")
        .arg("cd ndl-parser && go build -o ndl-parser main.go && cd ..")
        .output()?;

    println!("Build ndl-parser");

    Ok(())
}

fn directory_exists(dir_path: &str) -> bool {
    Path::new(dir_path).is_dir()
}

///////////// IGNORE BELOW FOR THIS BRANCH ////////////////////
fn get_contents_via_mut_buffer(file_path: &String) -> String {
    let mut data = String::new();
    let f = File::open(file_path).expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    data
}

fn main2() {
    let cmd = Command::parse();

    let mut lexer = Lexer::new(
        cmd.file_path.to_string(),
        get_contents_via_mut_buffer(&cmd.file_path),
    );
    let tokens = lexer.lex();

    for token in &tokens {
        println!("{}", token);
    }

    println!("Token len: {}", tokens.len());
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
