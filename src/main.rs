mod http;

use http::server::Server;

fn main() {
    let http_server = Server::new("127.0.0.1:8080", 8);

    http_server.run();

}
