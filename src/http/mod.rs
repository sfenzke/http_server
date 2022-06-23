mod request;
mod response;
pub mod server;

use crate::file::provider::FileProvider;

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
}