use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTION,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "PATCH" => Ok(Method::PATCH),
            "DELETE" => Ok(Method::DELETE),
            "OPTION" => Ok(Method::OPTION),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;
