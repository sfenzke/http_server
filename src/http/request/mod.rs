mod error;
mod method;

use std::convert::TryFrom;
use error::ParseError;
use method::Method;
use std::str;

/// A HTTP request
pub struct Request {
    /// The requested path
    path: String,
    /// The query string 
    query_string: Option<String>,
    /// the http method of the http request
    method: Method
}

impl Request {
    /// Helper method to split path from query_string
    /// # Arguments
    /// * `path_query_string` - A string slice containning the path and query_string
    /// 
    fn split_path_query_string(path_query_string: &str) -> Result<(String, Option<String>), ParseError> {
        let splitted_string:Vec<&str> = path_query_string.split("?").collect();

        match splitted_string.len() {
            1 => Ok((splitted_string[0].to_string(), None)),
            2 => Ok((splitted_string[0].to_string(), Some(splitted_string[1].to_string()))),
            _ => Err(ParseError::InvalidPath)
        }
    }
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
            3 => { 
                // split path from query string
                let splitted_string = Request::split_path_query_string(first_line[1])?;

                Ok(Request {
                    method: Method::try_from(first_line[0])?,
                    path: splitted_string.0,
                    query_string: splitted_string.1
                })
            },
            _ => Err(ParseError::InvalidRequest)
        }
    }   
}
