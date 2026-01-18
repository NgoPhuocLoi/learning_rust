use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process;

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

    let content = fs::read_to_string(config.file_path).expect("Can not read content of the file");
    dbg!(content);
}
