use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::search;

fn print_usage() {
    println!("minigrep <query_string> <file_path>")
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough args");
        }
        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|e| {
        print_usage();
        process::exit(1);
    });

    if let Err(err) = run(config) {
        println!("Error happened: {}", err);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    let result = search(&config.query, &content);
    dbg!(result);
    Ok(())
}
