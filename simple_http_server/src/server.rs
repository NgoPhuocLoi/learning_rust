use crate::http::{request::ParsedError, response::Response, status_code::StatusCode, Request};
use std::{io::Read, net::TcpListener};

pub trait RequestHandler {
    fn handle_request(&self, req: &Request) -> Response;
    fn handle_bad_request(&self, e: &ParsedError) -> Response {
        println!("Error happened: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

#[derive(Debug)]
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: &str) -> Self {
        Server {
            address: String::from(address),
        }
    }

    pub fn run(&self, handler: impl RequestHandler) {
        println!("Server is running on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let res = match Request::try_from(&buffer[..]) {
                                Ok(req) => {
                                    dbg!(&req);
                                    handler.handle_request(&req)
                                }
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            res.send(&mut stream);
                        }
                        Err(e) => {
                            println!("Error when reading a request, {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error happened: {}", e);
                }
            }
        }
    }
}
