use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::error::Error;
use std::str::Utf8Error;
use std::convert::From;
use super::method::MethodError;

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
    InvalidPath
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest    => "Invalid Request",
            Self::InvalidEncoding   => "Invalid Encoding",
            Self::InvalidProtocol   => "Invalid Protocol",
            Self::InvalidMethod     => "Invalid Method",
            Self::InvalidPath       => "Invalid Path"
        }
    }
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}