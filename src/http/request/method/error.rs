use std::error::Error;
use std::fmt::{Display, Debug, Result as FmtResult, Formatter};

#[derive(PartialEq, Eq, Debug)]
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