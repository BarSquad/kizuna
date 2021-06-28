use crate::udp::UdpError;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::io;

#[derive(Debug)]
pub enum ParsePacketErrorKind {
    InvalidSig,
    InvalidType,
    UnknownType,
    InvalidContent,
}

#[derive(Debug)]
pub struct ParsePacketError {
    pub kind: ParsePacketErrorKind,
}

impl ParsePacketError {
    pub fn new(kind: ParsePacketErrorKind) -> Self {
        Self { kind }
    }
}

impl fmt::Display for ParsePacketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParsePacketError {}

#[derive(Debug)]
pub enum HandlePacketError {
    IoError(io::Error),
    ParsePacketError(ParsePacketError),
}

impl From<io::Error> for HandlePacketError {
    fn from(err: io::Error) -> Self {
        HandlePacketError::IoError(err)
    }
}

impl From<ParsePacketError> for HandlePacketError {
    fn from(err: ParsePacketError) -> Self {
        HandlePacketError::ParsePacketError(err)
    }
}

impl fmt::Display for HandlePacketError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            HandlePacketError::IoError(err) => err.fmt(f),
            HandlePacketError::ParsePacketError(err) => err.fmt(f),
        }
    }
}

impl Error for HandlePacketError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            HandlePacketError::IoError(err) => err,
            HandlePacketError::ParsePacketError(err) => err,
        })
    }
}

impl From<ParsePacketError> for UdpError {
    fn from(err: ParsePacketError) -> Self {
        UdpError::new(HandlePacketError::ParsePacketError(err))
    }
}

impl From<HandlePacketError> for UdpError {
    fn from(err: HandlePacketError) -> Self {
        UdpError::new(err)
    }
}
