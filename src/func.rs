use crate::Error;

/// store closures
pub trait FnBox<P> {
    fn call(&mut self, props: &P) -> Result<(), Error>;
}
