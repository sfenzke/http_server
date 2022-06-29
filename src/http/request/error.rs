use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::error::Error;
use std::str::Utf8Error;
use std::convert::From;
use super::{method::MethodError};

pub enum ParseError {
    Request,
    Encoding,
    Protocol,
    Method,
    Path
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::Request    => "Invalid Request",
            Self::Encoding   => "Invalid Encoding",
            Self::Protocol   => "Invalid Protocol",
            Self::Method     => "Invalid Method",
            Self::Path       => "Invalid Path"
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
        Self::Encoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::Method
    }
}