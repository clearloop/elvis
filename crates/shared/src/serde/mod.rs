//! Serde node to target platforms
#[cfg(feature = "web")]
mod web;

/// data format transfer
pub trait Serde<S, T, E> {
    /// Serialize
    fn ser(&self) -> T;
}
