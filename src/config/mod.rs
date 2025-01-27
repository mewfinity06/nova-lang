use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    vec_starting_size: usize,
    vec_growth_factor: f64,
    dev_mode: bool,
    debug: bool,
}