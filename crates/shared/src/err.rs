//! Elvis shared errors
/// Elvis shared errors
#[derive(Debug)]
pub enum Error {
    /// Function Error
    FunctionError(String),
    /// Deserialize Error
    DeserializeHtmlError(String),
}
