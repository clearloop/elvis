//! Closure
use std::sync::Arc;

/// store closures
pub type Closure<T> = Arc<dyn Fn(T)>;
