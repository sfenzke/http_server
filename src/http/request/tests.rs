#[cfg(test)]
use super::ParseError;
#[cfg(test)]
use crate::http::method::Method;
#[cfg(test)]
use super::Request;
#[cfg(test)]
const CORRECT_REQUEST:&str = "GET index.html HTTP/1.1\r\nHost:localhost\r\n\r\n";

#[test]
fn origin_form() -> Result<(), ParseError> {
    let result = Request::try_from(CORRECT_REQUEST.as_bytes())?;
    println!("{result}");
    let expected = Request {
        path: String::from("index.html"),
        query_string: None,
        method: Method::Get
    };

    assert_eq!(expected, result);
    Ok(())
}
