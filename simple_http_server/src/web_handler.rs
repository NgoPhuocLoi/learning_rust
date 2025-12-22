use crate::http::{Request, Response, StatusCode};
use crate::server::RequestHandler;

pub struct WebHandler;

impl RequestHandler for WebHandler {
    fn handle_request(&self, req: &Request) -> Response {
        match req.path() {
            "/" => Response::new(StatusCode::Ok, Some(String::from("<h1>Hello aha</h1>"))),
            "/hello" => Response::new(StatusCode::Ok, Some(String::from("<h1>Hello AHDAD</h1>"))),
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
