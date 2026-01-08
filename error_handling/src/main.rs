use std::{
    fs::File,
    io::{self, Read},
};

fn unrecoverable_errors() {
    let x = 10;
    let y = 0;

    if y == 0 {
        panic!("What the hell bro??");
    }

    let result = x / y;

    println!("{x} + {y} = {result}");
}

fn open_file_and_propagate_error_if_exist(file_name: &str) -> Result<File, io::Error> {
    match File::open(file_name) {
        Ok(file) => Ok(file),
        Err(err) => Err(err),
    }
}

fn open_file_and_propagate_error_if_exist_shotcut(file_name: &str) -> Result<File, io::Error> {
    let file = File::open(file_name)?;
    Ok(file)
}

fn open_file_and_propagate_error_if_exist_shotcut_event_shorter(
    file_name: &str,
) -> Result<File, io::Error> {
    File::open(file_name)
}

fn recoverable_errors() {
    let f = File::open("hello.txt");

    match f {
        Ok(fi) => {
            println!("The file: {:?}", fi);
        }
        Err(e) => {
            println!("Error hannpend, {}", e);
        }
    }

    // let f2 = File::open("Hello.txtx").unwrap();
    // let f3 = File::open("Hello.txtx").expect("File may not be found");
    open_file_and_propagate_error_if_exist_shotcut("hello.txt").unwrap();
    open_file_and_propagate_error_if_exist("Hello there").unwrap();
}

fn main() {
    // unrecoverable_errors();
    recoverable_errors();
}
