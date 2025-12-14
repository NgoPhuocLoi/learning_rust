fn main() {
    let server = Server::new("127.0.0.1:8080");

    server.run();
}

struct Server {
    address: String,
}

impl Server {
    fn new(address: &str) -> Self {
        Server {
            address: String::from(address),
        }
    }

    fn run(&self) {
        println!("Server is running on {}", self.address);
    }
}
