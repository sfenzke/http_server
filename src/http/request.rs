mod error;

use std::convert::{From, TryFrom};

use error::ParseError;


pub struct Request {

}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        !unimplemented!()
    }   
}
