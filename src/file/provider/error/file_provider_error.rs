use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ErrorType {
    FILE_NOT_FOUND,
    READ_ERROR
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
           Self::FILE_NOT_FOUND => { write!(f, "File bot found")},
           Self::READ_ERROR => { write!(f, "Read error")}
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