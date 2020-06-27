use std::{convert::From, io::Error as IoError};
use toml::de::Error as TomlError;

/// Backend Errors
#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    TomlError(TomlError),
    Custom(String),
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::IoError(err)
    }
}

impl From<TomlError> for Error {
    fn from(err: TomlError) -> Error {
        Error::TomlError(err)
    }
}
