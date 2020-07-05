//! Elvis shared errors
/// Elvis shared errors
pub enum Error {
    /// Function Error
    FunctionError(String),
    /// Deserialize Error
    DeserializeHtmlError(String),
}
