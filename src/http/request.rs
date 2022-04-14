mod error;

use std::convert::TryFrom;
use error::ParseError;
use std::str;

enum Method {
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

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let req_data = str::from_utf8(buffer)?;
        
        let mut req_line_iterator = req_data.lines();
        let first_line = req_line_iterator.next().ok_or(ParseError::InvalidRequest)?;
        println!("{}", first_line);

        unimplemented!()
    }   
}
