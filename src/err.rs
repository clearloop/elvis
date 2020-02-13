/// Error sets
#[derive(Debug)]
pub enum Error {
    DeserializeHtmlError(String),
    SerializeHtmlError,
    NoneError,
}
