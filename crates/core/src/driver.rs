//! Evlis platform api driver

/// Elvis driver
///
/// Contains APIs for different platforms
pub trait Driver {
    /// Alert message
    fn alert(msg: &str);
    /// Common logger
    fn log(msg: &str);
}
