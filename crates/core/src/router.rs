//! Elvis Router
use crate::Error;

/// Elvis Router
pub trait Router {
    // /// Back to last page
    // fn back() -> Result<(), Error>;
    /// Push new path
    fn push(path: &str) -> Result<(), Error>;
}
