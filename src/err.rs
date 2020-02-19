/// Error sets
#[derive(Debug)]
pub enum Error {
    DeserializeHtmlError(String),
    FunctionError(String),
    SerializeHtmlError,
    NoneError,
}
