use core::fmt;
use std::str;

#[derive(Clone)]
pub enum Error {
    ParserError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParserError(ref record) => write!(fmt, "Bad Record: {}",record),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{self:}")
    }
}
