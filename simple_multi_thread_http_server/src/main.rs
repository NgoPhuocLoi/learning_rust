use std::{
    io::{BufRead, BufReader, Read},
    net::{TcpListener, TcpStream},
};

fn handle_connection(stream: TcpStream) {
    let buf = BufReader::new(&stream);
    let http_requests: Vec<String> = buf
        .lines()
        .map(|l| l.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();

    dbg!(http_requests);
}

fn main() {
    let listener = TcpListener::bind("localhost:9999").unwrap();

    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}
