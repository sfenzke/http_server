use std::path::Path;

use crate::file::provider::FileProvider;

use super::{response::Response, request::Request};

pub struct RequestHandler<T: FileProvider> {
    file_provider: T
}

impl<T:FileProvider> RequestHandler<T> {
    /// Creates a new [`RequestHandler<T>`].
    pub fn new(file_provider: T) -> Self {
        Self {
            file_provider,
        }
    }

    fn map_index(&self, file: &String) -> String {
        match file.as_str() {
            "/" => String::from("index.html"),
            _ => {
                let mut chars = file.chars();
                chars.next();
                chars.as_str().to_owned()
            }
        }
    }

    pub fn handle_request(&self, request:&Request) -> Response {

        match self.file_provider.provide_file(&(self.map_index(&request.path))) {
            Ok(data) => Response { },
            Err(e) => { println!("{:?}", e); return Response { } }
        }
    }
}