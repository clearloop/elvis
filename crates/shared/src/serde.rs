//! Serde trait

/// data format transfer
pub trait Serde<S, T, E> {
    /// Serialize
    fn ser(&self) -> T;
}
