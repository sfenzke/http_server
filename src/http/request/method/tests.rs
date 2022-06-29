#[cfg(test)]
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