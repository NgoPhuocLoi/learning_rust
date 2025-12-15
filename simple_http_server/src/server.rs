use std::{io::Read, net::TcpListener};

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
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let request_text = String::from_utf8_lossy(&buffer);
                            println!("Receiving a request....");
                            println!("{}", request_text);
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
