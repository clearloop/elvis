//! Elvisjs
#![warn(missing_docs)]
mod bridge;
mod err;

// Exports widgets
pub mod widgets;

// Exports idents
pub use crate::err::Error;

/// Gesture module
pub mod gesture {
    pub use elvis_core::{Gesture, GestureDetector};
}

pub use elvis_core::{style, value};

/// Evlis Traits
pub mod traits {
    pub use elvis_core::{Driver as DriverTrait, Router as RouterTrait};
}

// Driver
#[cfg(feature = "web")]
pub use elvis_web::{Driver, Router};

/// A module which is typically glob imported from:
///
/// ```
/// use elvis::prelude::*;
/// ```
pub mod prelude {
    pub use elvis_core::{LifeCycle, Node};
    pub use elvis_derive::page;
    #[cfg(feature = "web")]
    pub use elvis_web::{wasm_bindgen_re_exports::*, Page};
}
