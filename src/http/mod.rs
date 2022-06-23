mod request;
mod response;
pub mod server;

use crate::file::provider::FileProvider;
use request::Request;
use response::Response;

struct RequestHandler<T: FileProvider> {
    file_provider: T
}

impl<T:FileProvider> RequestHandler<T> {
    /// Creates a new [`RequestHandler<T>`].
    pub fn new(file_provider: T) -> Self {
        RequestHandler { 
            file_provider,
        }
    }

    pub fn handle(&self, request: Request) -> Response {
        println!("RequestHandler: {}", request);
        self.file_provider.provide_file(request.path);
        Response {}
    }
}