use crate::Error;

/// data format transfer
pub trait Serde<S, T> {
    fn de(h: T) -> Result<S, Error>;
    fn ser(&self) -> T;
}
