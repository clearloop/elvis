use crate::Error;

/// data format transfer, use `String` temporarily
pub trait Parser<S, T> {
    fn de(h: T) -> Result<S, Error>;
    fn ser(self) -> T;
}
