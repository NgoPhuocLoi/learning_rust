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

    let path = Path::new(file_path);
    let file = fs::File::open(path);

    match file {
        Ok(mut f) => {
            let mut buf = String::new();
            f.read_to_string(&mut buf).expect("Can not read the file");
            dbg!(buf);
        }
        Err(_) => {
            panic!("File not found");
        }
    }
}
