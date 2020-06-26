use std::{convert::From, io::Error as IoError};

/// Backend Errors
#[derive(Debug)]
pub enum Error {
    IoError(IoError),
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::IoError(err)
    }
}
