//! This crate implements a http server. 

mod http;
mod file;
mod server;

use file::provider::{SimpleFilesystemProvider};
use server::Server;

/// This is the main function which creates the Server and starts the main loop on it.
fn main() {
    let http_server = Server::new("127.0.0.1:8080", 
                                                                    SimpleFilesystemProvider::new(String::from("web_base_test")));

    http_server.run();

}
