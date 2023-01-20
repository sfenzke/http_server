use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ErrorType {
    FileNotFound,
    ReadError
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
           Self::FileNotFound => { write!(f, "File bot found")},
           Self::ReadError => { write!(f, "Read error")}
        }
    }
}

#[derive(Debug)]
pub struct FileProviderError {
    message: String,
    error_type: ErrorType
}

impl FileProviderError {
    pub fn new(message: String, error_type: ErrorType ) -> Self {
        Self {
            message,
            error_type
        }
    }
}

impl Error for FileProviderError {}

impl Display for FileProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error_type, self.message)
    }
}