use crate::http::{Method, Request, Response, StatusCode};
use crate::server::RequestHandler;
use std::fs;

pub struct WebHandler {
    public_path: String,
}

impl WebHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let full_path = format!("{}{}", self.public_path, file_path);
        println!("Reading file: {}", full_path);
        match fs::canonicalize(full_path) {
            Ok(path) => {
                println!("Pathh: {}", path.display());
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    Some(String::from("Not found"))
                }
            }
            Err(_) => Some(String::from("Not found error")),
        }
    }
}

impl RequestHandler for WebHandler {
    fn handle_request(&self, req: &Request) -> Response {
        match req.method() {
            Method::GET => match req.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("/index.html")),
                path => Response::new(StatusCode::Ok, self.read_file(path)),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
