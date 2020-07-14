//! Closure
// use crate::err::Error;

/// store closures
pub type Closure<T> = dyn FnMut(T) -> Option<()>;
