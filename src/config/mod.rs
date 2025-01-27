use std::{path::Path, process::Command};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    vec_starting_size: usize,
    vec_growth_factor: f64,
    dev_mode: bool,
    debug: bool,
    output_file: String,
}

pub fn get_config() -> anyhow::Result<Config> {
    ///////////// Build NDL Parser ///////////////////////

    // May not work on other operating systems from Linux
    let dir_path = "ndl-parser"; // The directory that ndl-parser should live in
    let exe_name = "ndl-parser"; // Linux executables usually don’t have ".exe"

    let exe_path = Path::new(dir_path).join(exe_name);

    if !directory_exists(dir_path) {
        get_ndl_parser_from_github()?;
    } else {
        update_ndl_parser()?;
    }

    // If ndl-parser does not exits
    if !exe_path.exists() && !exe_path.is_file() {
        build_ndl_parser()?;
    }

    ///////////// Use NDL Parser ///////////////////////

    let ndl_parser = Command::new(exe_path).arg(".").output()?;

    let ndl_parsed = format!("{}", String::from_utf8_lossy(&ndl_parser.stdout));

    let def: Config = serde_json::from_str(&ndl_parsed)?;
    Ok(def)
}

fn get_ndl_parser_from_github() -> anyhow::Result<()> {
    println!("Getting ndl-parser from Github");

    Command::new("sh")
        .arg("-c")
        .arg("git clone https://github.com/mewfinity06/ndl-parser.git")
        .output()?;

    Ok(())
}

fn build_ndl_parser() -> anyhow::Result<()> {
    println!("Building ndl-parser");

    Command::new("sh")
        .arg("-c")
        .arg("cd ndl-parser && go build -o ndl-parser main.go && cd ..")
        .output()?;

    Ok(())
}

fn directory_exists(dir_path: &str) -> bool {
    Path::new(dir_path).is_dir()
}

fn update_ndl_parser() -> anyhow::Result<()> {
    Command::new("sh")
        .arg("-c")
        .arg("cd ndl-parser && git pull")
        .output()?;

    Ok(())
}
