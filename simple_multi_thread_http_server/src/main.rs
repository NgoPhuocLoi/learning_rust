use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    let buf = BufReader::new(&stream);
    let http_requests: Vec<String> = buf
        .lines()
        .map(|l| l.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();

    let first_line = &http_requests[0];

    println!("{first_line}");

    let (status_line, file_name) = if first_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 400 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(file_name).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("localhost:9999").unwrap();

    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}
