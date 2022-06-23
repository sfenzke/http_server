use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};

pub enum ResponseError {
    FILE_NOT_FOUND
}

impl ResponseError {
    fn message(&self) -> &str{
        match *self {
            FILE_NOT_FOUND => "File net found"
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