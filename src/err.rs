use crate::tree::parser::Extra;

/// Errors
#[derive(Debug)]
pub enum Error {
    DeserializeHtmlError(String),
    ExtraLeft(Extra),
    SerializeHtmlError,
    NoneError,
}

impl std::convert::From<std::option::NoneError> for Error {
    fn from(_: std::option::NoneError) -> Error {
        Error::DeserializeHtmlError("unwrap on None type".to_string())
    }
}
