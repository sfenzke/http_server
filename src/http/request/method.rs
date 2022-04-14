use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Result as FmtResult, Formatter};

pub enum Method {
    GET,
    POST,
    PUT,
    HEAD,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

impl TryFrom<&str> for Method {
    type Error = MethodError;  

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET"       => Ok(Self::GET),
            "POST"      => Ok(Self::POST),
            "PUT"       => Ok(Self::PUT),
            "HEAD"      => Ok(Self::HEAD),
            "DELETE"    => Ok(Self::DELETE),
            "CONNECT"   => Ok(Self::CONNECT),
            "OPTIONS"   => Ok(Self::OPTIONS),
            "TRACE"     => Ok(Self::TRACE),
            "PATCH"     => Ok(Self::PATCH),
            _           => Err(MethodError::new(format!("{} is not a valid HTTP method", value).to_string()))
        }
    }
}

 pub struct MethodError {
    message: String

}

impl MethodError {
    fn new(message: String) -> Self {
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