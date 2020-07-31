//! Elvis shared errors
/// Elvis shared errors
#[derive(Debug)]
pub enum Error {
    /// Function Error
    FunctionError(String),
    /// Deserialize Error
    DeserializeHtmlError(String),
    /// Router Error
    RouterError(String),
    /// Custom Error
    Custom(String),
}

impl Error {
    /// Check derive errors
    pub fn check<T, E, IE>(r: &Result<T, IE>) -> Result<(), Error>
    where
        E: Into<Error> + From<IE>,
        IE: Clone,
    {
        if let Err(e) = r {
            Err(E::from(e.clone()).into())
        } else {
            Ok(())
        }
    }
}
