use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct FileProviderError {
    message: String
}

impl FileProviderError {
    pub fn new(message: String) -> Self {
        Self {
            message
        }
    }
}

impl Error for FileProviderError {}

impl Display for FileProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}