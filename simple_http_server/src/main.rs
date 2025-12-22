use server::Server;

use web_handler::WebHandler;
mod http;
mod server;
mod web_handler;

fn main() {
    let server = Server::new("127.0.0.1:8080");
    println!("Server struct: {server:#?}");
    server.run(WebHandler);
}
