use std::env;
use std::process;

use minigrep;
use minigrep::Config;

fn main() {
    // 1. receive args
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem: {}", err);
        process::exit(1);
    });

    println!("query: {}", config.query);
    println!("filename: {}", config.filename);

    // 2. read file
    if let Err(e) = minigrep::run(config) {
        println!("error: {}", e);
        process::exit(1);
    }
}
