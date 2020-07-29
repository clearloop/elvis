use etc::Error as EtcError;
use notify::Error as NotifyError;
use std::{convert::From, io::Error as IoError};
use toml::de::Error as TomlError;

/// Backend Errors
#[derive(Debug)]
pub enum Error {
    /// Io error
    IoError(IoError),
    /// Toml error
    TomlError(TomlError),
    /// Notify error
    NotifyError(NotifyError),
    /// Etc error
    EtcError(EtcError),
    /// Custom string error
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

impl From<NotifyError> for Error {
    fn from(err: NotifyError) -> Error {
        Error::NotifyError(err)
    }
}

impl From<EtcError> for Error {
    fn from(err: EtcError) -> Error {
        Error::EtcError(err)
    }
}
