#[cfg(feature = "web")]
use crate::tree::web::Extra;

/// Errors
#[derive(Debug)]
pub enum Error {
    DeserializeHtmlError(String),
    #[cfg(feature = "web")]
    ExtraLeft(Extra),
    SerializeHtmlError,
    NoneError,
}

impl std::convert::From<std::option::NoneError> for Error {
    fn from(_: std::option::NoneError) -> Error {
        Error::DeserializeHtmlError("unwrap on None type".to_string())
    }
}
