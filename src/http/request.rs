use super::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};

pub struct Request {
    path:String,
    query_string:Option<String>,
    method:Method
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buff: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidMethod,
    InvalidBuffer,
    InvalidProtocol
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidMethod => "InvalidMethod",
            Self::InvalidBuffer => "InvalidBuffer",
            Self::InvalidProtocol => "InvalidProtocol",
        }
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