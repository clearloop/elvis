//! Elvisjs
#![warn(missing_docs)]
mod bridge;
mod err;

/// Imports structs from elvis_core
use elvis_core::{Node, Serde};

// Exports modules
pub mod widgets;

// Export idents
pub use crate::err::Error;
/// Gesture module
pub mod gesture {
    pub use elvis_core::{Gesture, GestureDetector};
}

/// Elvis drivers
pub mod driver {
    pub use elvis_core::Driver;
    #[cfg(feature = "web")]
    pub use elvis_web::WebDriver;
}

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

// elvis platform features
/// Browser data bridge, as default feature.
#[cfg(feature = "web")]
#[path = "platforms/web/mod.rs"]
mod web;
