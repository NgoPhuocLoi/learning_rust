use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::search;
use minigrep::search_case_insensitive;

fn print_usage() {
    println!("minigrep <query_string> <file_path>")
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough args");
        }
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        print_usage();
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Error happened: {}", err);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    for line in result {
        println!("{line}");
    }
    Ok(())
}
