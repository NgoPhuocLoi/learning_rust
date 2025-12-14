use std::net::TcpListener;

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

    pub fn run(&self) {
        println!("Server is running on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((stream, _)) => {
                    println!("The comming stream: {stream:?}");
                }
                Err(e) => {
                    println!("Error happened: {}", e);
                }
            }
        }
    }
}
