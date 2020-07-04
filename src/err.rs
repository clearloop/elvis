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
