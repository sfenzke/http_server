mod error;
mod tests;
use error::ParseError;
use std::{fmt::{Display, Debug, Formatter, Result as FmtResult}, str, convert::TryFrom};
use crate::http::method::Method;



/// A HTTP request
#[derive(PartialEq, Eq)]
pub struct Request {
    /// The requested path
    pub path: String,
    /// The query string 
    pub query_string: Option<String>,
    /// the http method of the http request
    pub method: Method
}

impl Request {
    /// Helper method to split path from query_string
    /// # Arguments
    /// * `path_query_string` - A string slice containning the path and query_string
    /// 
    fn split_path_query_string(path_query_string: &str) -> Result<(String, Option<String>), ParseError> {
        let splitted_string:Vec<&str> = path_query_string.split('?').collect();

        match splitted_string.len() {
            1 => Ok((splitted_string[0].to_string(), None)),
            2 => Ok((splitted_string[0].to_string(), Some(splitted_string[1].to_string()))),
            _ => Err(ParseError::Path)
        }
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(qs) = &self.query_string {
            write!(f, "path: {}, query_string: {}, method: {}", 
                self.path, qs, self.method)
        }
        else {
            write!(f, "path: {}, method: {}", 
                self.path, self.method)
        }
    }
}

impl Debug for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(qs) = &self.query_string {
            write!(f, "path: {}, query_string: {}, method: {}", 
                self.path, qs, self.method)
        }
        else {
            write!(f, "path: {}, method: {}", 
                self.path, self.method)
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
                            ok_or(ParseError::Request)?).
                            split_ascii_whitespace().
                            collect::<Vec<&str>>();
        
        // if there are not exactly 3 elements in the vector containing
        // the elements of the first line, the request is invalid
        if first_line.len() != 3 { return Err(ParseError::Request); }
        // we only support HTTP/1.1 at the moment
        if first_line[2] != "HTTP/1.1" { return Err(ParseError::Protocol); }
        // split path from query string
        let splitted_string = Request::split_path_query_string(first_line[1])?;

        Ok(Request {
            method: Method::try_from(first_line[0])?,
            path: splitted_string.0,
            query_string: splitted_string.1
        })
    }   
}