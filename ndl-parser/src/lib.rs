use glob::glob;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    pub vec_starting_size: i32, // Default: 0
    pub vec_growth_factor: f64, //        : 2
    pub dev_mode: bool,         //        : false
    pub debug: bool,            //        : false
    pub output_file: String,    //        : Main
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vec_starting_size: 0,
            vec_growth_factor: 2f64,
            dev_mode: false,
            debug: false,
            output_file: String::from("Main"),
        }
    }
}

pub fn parse_config(filename: &str) -> Config {
    let mut config = Config::default();

    if filename.is_empty() {
        return config;
    }

    let file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => return config,
    };

    let reader = BufReader::new(file);
    let config_pattern = Regex::new(r"\[(.*?)\](.*)").unwrap();

    for line in reader.lines() {
        if let Ok(line) = line {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some(caps) = config_pattern.captures(line) {
                let key = &caps[1];
                let value = caps[2].trim().trim_end_matches(';');

                match key {
                    "vec_starting_size" => {
                        if let Ok(v) = value.parse::<i32>() {
                            config.vec_starting_size = v;
                        }
                    }
                    "vec_growth_factor" => {
                        if let Ok(v) = value.parse::<f64>() {
                            config.vec_growth_factor = v;
                        }
                    }
                    "output" => config.output_file = value.to_string(),
                    "dev_mode" => config.dev_mode = value == "true",
                    "debug" => config.debug = value == "true",
                    _ => {}
                }
            }
        }
    }

    config
}

pub fn get_config(config_dir: &str) -> Option<Config> {
    let pattern = format!("{}/*.ndl", config_dir);
    let paths = match glob(&pattern) {
        Ok(paths) => paths.filter_map(Result::ok).collect::<Vec<_>>(),
        Err(_) => Vec::new(),
    };

    if paths.is_empty() {
        return None;
    }

    let config = parse_config(paths[0].to_str().unwrap());
    Some(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use tempfile::tempdir;

    #[test]
    fn test_parse_config_with_valid_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.ndl");

        let config_content = "
            [vec_starting_size] 100;
            [vec_growth_factor] 2.0;
            [output] test_output;
            [dev_mode] true;
            [debug] false;
        ";

        write(&file_path, config_content).unwrap();
        let config = parse_config(file_path.to_str().unwrap());

        assert_eq!(config.vec_starting_size, 100);
        assert_eq!(config.vec_growth_factor, 2.0);
        assert_eq!(config.output_file, "test_output");
        assert_eq!(config.dev_mode, true);
        assert_eq!(config.debug, false);
    }

    #[test]
    fn test_parse_config_with_empty_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("empty.ndl");

        write(&file_path, "").unwrap();
        let config = parse_config(file_path.to_str().unwrap());

        assert_eq!(config, Config::default());
    }

    #[test]
    fn test_get_config_with_existing_ndl_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.ndl");

        let config_content = "
            [vec_starting_size] 50;
            [vec_growth_factor] 1.75;
            [output] output_file;
            [dev_mode] false;
            [debug] true;
        ";

        write(&file_path, config_content).unwrap();
        let config = get_config(dir.path().to_str().unwrap()).unwrap();

        assert_eq!(config.vec_starting_size, 50);
        assert_eq!(config.vec_growth_factor, 1.75);
        assert_eq!(config.output_file, "output_file");
        assert_eq!(config.dev_mode, false);
        assert_eq!(config.debug, true);
    }

    #[test]
    fn test_get_config_with_no_ndl_file() {
        let dir = tempdir().unwrap();
        let config = get_config(dir.path().to_str().unwrap());
        assert!(config.is_none());
    }
}
