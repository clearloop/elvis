use crate::Error;

/// store closures
pub trait FnBox {
    fn call_box(self: Box<Self>) -> Result<(), Error>;
}
