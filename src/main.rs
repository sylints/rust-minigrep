use std::env;
use std::fs;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    println!("{} {}", config.query, config.filename);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(x: &[String]) -> Config {
    let query = x[1].clone();
    let filename = x[2].clone();

    Config { query, filename }
}
