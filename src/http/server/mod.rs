use std::io::Read;
use std::net::TcpListener;
use super::request::Request;

pub struct Server {
    bind_addr: String,
}

impl Server {
    pub fn new(bind_addr: &str) -> Self {
        Self {
            bind_addr: String::from(bind_addr),
        }
    }

    pub fn run(self) {
        let connection = TcpListener::bind(self.bind_addr).unwrap();

        loop {
            match connection.accept(){
                Ok((mut stream, source_addr)) => {
                    println!("Connected to {}", source_addr);

                    let mut buffer:[u8; 1024] = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            //println!("{}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    // handle request here
                                    println!("{}", request);
                                }
                                Err(e) => {
                                    // Some kind of error happened and needs to be handled here
                                    println!("Damn! {}", e);
                                }
                            }
                        },
                        Err(e) => println!("Failed to read data: {}", e)
                    };
                },
                Err(e) => println!("Failed to accept connection: {}", e)
            };
        }
    }
}