use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {}

impl Config {
    pub fn new(path: &str) -> Self {
        let config = fs::read_to_string(path).expect("Could not find config.toml");
        todo!()
    }
}
