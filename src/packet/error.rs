use std::io;

#[derive(Debug)]
pub enum ParsePacketErrorKind {
    InvalidSig,
    InvalidType,
    UnknownType,
    InvalidContent,
}

// TODO: Реализовать Error
#[derive(Debug)]
pub struct ParsePacketError {
    pub kind: ParsePacketErrorKind,
}

impl ParsePacketError {
    pub fn new(kind: ParsePacketErrorKind) -> Self {
        Self { kind }
    }
}

// TODO: Реализовать Error
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
