use std::error::Error;
use std::fmt::{Display, Debug, Result as FmtResult, Formatter};

pub struct MethodError {
    message: String

}

impl MethodError {
    pub fn new(message: String) -> Self {
        Self {
            message
        }
    }
}

impl Error for MethodError {}

impl Display for MethodError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "{}", self.message)
    }
}

impl Debug for MethodError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message)
    }
}