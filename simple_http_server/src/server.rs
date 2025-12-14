pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: &str) -> Self {
        Server {
            address: String::from(address),
        }
    }

    pub fn run(&self) {
        println!("Server is running on {}", self.address);
    }
}
