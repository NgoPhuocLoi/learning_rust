use super::Method;
use std::fmt::Result as FmtResult;
use std::str;
use std::{convert::TryFrom, error::Error, fmt::Display, fmt::Formatter};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParsedError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(value).or(Err(ParsedError::InvalidEncoding))?;
        let (method, request) = get_next_word(request).ok_or(ParsedError::InvalidEncoding)?;
        let (path, request) = get_next_word(request).ok_or(ParsedError::InvalidEncoding)?;
        let (protocol, _) = get_next_word(request).ok_or(ParsedError::InvalidEncoding)?;

        if protocol != "HTTP/1.1" {
            return Err(ParsedError::InvalidProtocal);
        }

        unimplemented!();
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

#[derive(Debug)]
pub enum ParsedError {
    InvalidRequest,
    InvalidEncoding,
    InvalidMethod,
    InvalidProtocal,
}

impl ParsedError {
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidMethod => "InvalidMethod",
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidProtocal => "InvalidProtocal",
        }
    }
}

impl Display for ParsedError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParsedError {}
