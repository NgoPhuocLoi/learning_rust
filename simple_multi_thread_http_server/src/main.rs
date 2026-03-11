use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::Path,
};

fn handle_connection(mut stream: TcpStream) {
    let buf = BufReader::new(&stream);
    let http_requests: Vec<String> = buf
        .lines()
        .map(|l| l.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();

    dbg!(http_requests);

    let status_line = "HTTP/1.1 200 OK";
    let content = fs::read_to_string(Path::new("hello.html")).unwrap();
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
