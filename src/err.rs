//! Elvis errors
use elvis_shared::Error as SharedError;

/// Error sets
#[derive(Debug)]
pub enum Error {
    /// Deserialize html plain text failed
    DeserializeHtmlError(String),
    /// Passing in broken function
    FunctionError(String),
    /// Serialize widgets to html failed
    SerializeHtmlError,
    /// Unwrap a none value
    NoneError,
}

impl From<SharedError> for Error {
    fn from(e: SharedError) -> Error {
        match e {
            SharedError::FunctionError(s) => Error::FunctionError(s),
            SharedError::DeserializeHtmlError(s) => Error::DeserializeHtmlError(s),
        }
    }
}
