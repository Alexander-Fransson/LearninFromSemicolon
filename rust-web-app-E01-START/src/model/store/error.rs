use core::fmt::Formatter;
use std::fmt::write;    
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, Clone)]
pub enum Error {
    FailedToCreatePool(String)
}

impl core::fmt::Display for Error {
    fn fmt(
        &self, 
        fmt: &mut Formatter
    ) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}