use crate::http::method::MethodError;

use super::Method;
use std::fmt::Result as FmtResult;
use std::str::{self, Utf8Error};
use std::{convert::TryFrom, error::Error, fmt::Display, fmt::Formatter};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParsedError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(value)?;
        let (method, request) = get_next_word(request).ok_or(ParsedError::InvalidEncoding)?;
        let (mut path, request) = get_next_word(request).ok_or(ParsedError::InvalidEncoding)?;
        let (protocol, _) = get_next_word(request).ok_or(ParsedError::InvalidEncoding)?;

        if protocol != "HTTP/1.1" {
            return Err(ParsedError::InvalidProtocal);
        }

        let method: Method = method.parse()?;

        let mut q = None;
        if let Some(i) = path.find('?') {
            q = Some(&path[i + 1..]);
            path = &path[..i];
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

impl From<Utf8Error> for ParsedError {
    fn from(_: Utf8Error) -> Self {
        ParsedError::InvalidEncoding
    }
}

impl From<MethodError> for ParsedError {
    fn from(_: MethodError) -> Self {
        ParsedError::InvalidMethod
    }
}

impl Display for ParsedError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParsedError {}
