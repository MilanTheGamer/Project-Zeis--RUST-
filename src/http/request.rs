use super::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

pub struct Request {
    path:String,
    query_string:Option<String>,
    method:Method
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buff: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buff)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidMethod)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidMethod)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidMethod)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol)
        }

        unimplemented!()
    }
}

fn get_next_word(request:&str) -> Option<(&str,&str)> {
    for (index,character) in request.chars().enumerate() {
        if character == ' ' || character == '\r' {
            return Some((&request[..index],&request[index+1..]))
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidMethod,
    InvalidEncoding,
    InvalidProtocol
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidMethod => "InvalidMethod",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_:Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f,"{}",self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f,"{}",self.message())
    }
}

impl Error for ParseError {}