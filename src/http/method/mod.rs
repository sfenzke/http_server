mod error;

mod tests;
pub(crate) use error::MethodError;

use std::convert::TryFrom;
use std::fmt::{Display, Debug, Result as FmtResult, Formatter};

#[derive(PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Head,
    Delete,
    Connect,
    Options,
    Trace,
    Patch
}

impl TryFrom<&str> for Method {
    type Error = MethodError;  

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET"       => Ok(Self::Get),
            "POST"      => Ok(Self::Post),
            "PUT"       => Ok(Self::Put),
            "HEAD"      => Ok(Self::Head),
            "DELETE"    => Ok(Self::Delete),
            "CONNECT"   => Ok(Self::Connect),
            "OPTIONS"   => Ok(Self::Options),
            "TRACE"     => Ok(Self::Trace),
            "PATCH"     => Ok(Self::Patch),
            _           => Err(MethodError::new(format!("{} is not a valid HTTP method", value)))
        }
    }
}

impl Method {
    fn to_str(&self) -> &str {
        match *self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Head => "HEAD",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
            Method::Patch => "PATCH",
        }
    }
}


impl Display for Method {
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "{}", self.to_str())
    }
}

impl Debug for Method {
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "{}", self.to_str())
    }
}