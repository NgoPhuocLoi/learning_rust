use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

fn print_usage() {
    println!("minigrep <query_string> <file_path>")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        print_usage();
        return;
    }

    let query = &args[1];
    let file_path = &args[2];

    let content = fs::read_to_string(file_path).expect("Can not read content of the file");
    dbg!(content);
}
