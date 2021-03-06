use crate::file::provider::FileProvider;

use super::{response::Response, request::Request};

pub struct RequestHandler<T: FileProvider> {
    file_provider: T
}

impl<T:FileProvider> RequestHandler<T> {
    /// Creates a new [`RequestHandler<T>`].
    pub fn new(file_provider: T) -> Self {
        RequestHandler { 
            file_provider,
        }
    }

    pub fn handle_request(&self, request:&Request) -> Response {
        self.file_provider.provide_file(&request.path);
        Response {  }
    } 
}