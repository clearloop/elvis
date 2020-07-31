//! Web errors
use elvis_core::Error as CoreError;
use wasm_bindgen::JsValue;

/// Web Errors
pub enum Error {
    /// Error from js value
    JavascriptError(String),
}

impl From<JsValue> for Error {
    fn from(e: JsValue) -> Error {
        Error::JavascriptError(
            e.as_string()
                .unwrap_or_else(|| "unknown javascript error".into()),
        )
    }
}

impl Into<CoreError> for Error {
    fn into(self) -> CoreError {
        match self {
            Error::JavascriptError(s) => CoreError::Custom(s),
        }
    }
}
