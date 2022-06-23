mod http;
mod file;

use http::server::Server;
use file::provider::DummyProvider;

fn main() {
    let http_server = Server::new("127.0.0.1:8080", DummyProvider::new());

    http_server.run();

}
