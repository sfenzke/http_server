use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};

pub enum ResponseError {
    FileNotFound
}

impl ResponseError {
    fn message(&self) -> &str{
        match self {
            ResponseError::FileNotFound => "File not found"
        }
    }
}

impl Error for ResponseError {}

impl Display for ResponseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ResponseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}