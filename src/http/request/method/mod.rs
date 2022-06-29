mod error;
pub(crate) use error::MethodError;

use std::convert::TryFrom;
use std::fmt::{Display, Debug, Result as FmtResult, Formatter};

#[derive(PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::{Method, MethodError};

    #[test]
    fn convert_str_to_method() -> Result<(), MethodError> {
        let result = Method::try_from("GET")?;
        assert_eq!(result, Method::Get);
        Ok(())
    }

    #[test]
    fn fail_when_method_is_not_uppercase() {
        let result = Method::try_from("get");
        let expected = Err(MethodError::new(String::from("get is not a valid HTTP method")));
        assert_eq!(result, expected);
    }
}