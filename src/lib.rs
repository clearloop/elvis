//! Elvisjs
#![warn(missing_docs)]
// mod bridge;
mod err;

pub mod gesture;
pub mod prelude;
pub mod traits;
pub mod widgets;

// re-exports
pub use elvis_core::{style, value};

// self exports
pub use crate::err::Error;

// web features
#[cfg(feature = "web")]
pub use elvis_web::{Driver, Router};
