mod error;
mod method;

use std::convert::TryFrom;
use error::ParseError;
use method::Method;
use std::str;

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

        // get the first line from the request header and split in into segments
        // it should look like: GET /?id=0 HTTP/1.1
        let first_line = (req_line_iterator.next().
                            ok_or(ParseError::InvalidRequest)?).
                            split_ascii_whitespace().
                            collect::<Vec<&str>>();
        
        // if there are not exactly 3 elements in the vector containing
        // the elements of the first line, the request is invalid
        match first_line.len() {
            3 => Ok(Request {
                    method: Method::try_from(first_line[0])?,
                    path: first_line[1].to_string(),
                    query_string: None
            }),
            _ => Err(ParseError::InvalidRequest)
        }
    }   
}
