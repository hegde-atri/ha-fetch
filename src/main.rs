use clap::{Arg, Command};
use std::path::Path;

mod config;
mod stats;

fn main() {
    let matches = Command::new("ha-fetch")
        .version("0.1")
        .author("Atri Hegde <iamatrihegde@outlook.com>")
        .about("Another fetch script, written in rust")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("CONFIG")
                .help("Provide path to config"),
        )
        .get_matches();

    // Check for HOME path, and then configuration file.
    // If configuration missing, proceed with default config.
}
