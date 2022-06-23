mod http;
mod file;

use http::server::Server;

fn main() {
    let http_server = Server::new("127.0.0.1:8080");

    http_server.run();

}
