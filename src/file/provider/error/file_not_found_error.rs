use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct FileNotFoundError {
    message: String
}

impl FileNotFoundError {
    pub fn new(message: String) -> Self {
        Self {
            message
        }
    }
}

impl Error for FileNotFoundError {}

impl Display for FileNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}