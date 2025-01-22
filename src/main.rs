use clap::Parser;

fn main() {
    let cmd = Command::parse();

    println!("{:?}", cmd);
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
