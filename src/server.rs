use std::io::Read;
use std::net::TcpListener;
use crate::{http::{RequestHandler, request::Request}, file::provider::FileProvider};

pub struct Server<T: crate::file::provider::FileProvider> {
    bind_addr: String,
    request_handler: RequestHandler<T>,
}

impl<T> Server<T> where T: FileProvider {
    /**
    creates an new [`Server`]
    */
    pub fn new(bind_addr: &str, file_provider: T) -> Self {
        Self {
            bind_addr: String::from(bind_addr),
            request_handler: RequestHandler::new(file_provider)
        }
    }

    /// main loop of the [`Server`]
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
                                    self.request_handler.handle_request(&request);
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