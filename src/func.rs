use crate::Error;

/// store closures
pub trait FnBox {
    fn call(&mut self) -> Result<(), Error>;
}
