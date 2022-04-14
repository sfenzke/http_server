mod http;

use http::Server;

fn main() {
    let http_server = Server::new("127.0.0.1:8080".to_string(), 8);

    http_server.run();
}