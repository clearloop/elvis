/// data format transfer
pub trait Serde<S, T, E> {
    /// Deserialize
    fn de(h: T) -> Result<S, E>;
    /// Serialize
    fn ser(&self) -> T;
}
