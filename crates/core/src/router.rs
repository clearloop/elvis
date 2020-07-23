//! Elvis Router
use crate::Error;

/// Elvis Router
pub trait Router {
    /// Back to last page
    fn back() -> Result<(), Error>;

    /// Push new path
    fn push(path: &str, title: &str) -> Result<(), Error>;

    /// Replace current route
    fn replace(path: &str, title: &str) -> Result<(), Error>;

    /// Register page to name
    fn register<P>(&mut self, path: String, page: P);

    /// Get specific route
    fn route<P>(&mut self, path: String, page: P) -> &mut P;
}
