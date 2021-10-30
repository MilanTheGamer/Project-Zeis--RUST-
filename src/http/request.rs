use super::{Method, MethodError};
use super::{QueryString};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path:&'buf str,
    query_string:Option<QueryString<'buf>>,
    method:Method
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buff: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buff)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidMethod)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidMethod)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidMethod)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol)
        }

        let method : Method = method.parse()?;

        let mut query_string = None;

        if let Some(index) = path.find('?'){
            query_string = Some(QueryString::from(&path[index+1..]));
            path = &path[..index];
        }

        Ok(Self {
            path,
            query_string,
            method
        })
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

impl From<MethodError> for ParseError {
    fn from(_:MethodError) -> Self {
        Self::InvalidMethod
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