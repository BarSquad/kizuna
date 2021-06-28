use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct UdpError {
    err: Box<dyn Error + Send + Sync>,
}

impl UdpError {
    pub fn new<E: Error + Send + Sync + 'static>(err: E) -> Self {
        Self { err: Box::new(err) }
    }
}

impl fmt::Display for UdpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for UdpError {}
