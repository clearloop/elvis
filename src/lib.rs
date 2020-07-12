//! Elvisjs
#![warn(missing_docs)]
mod bridge;
mod err;
pub mod gestures;
pub mod widgets;

/// Imports structs from elvis_shared
use elvis_shared::{Node, Serde};

// Exports
pub use crate::{
    err::Error,
    widgets::{layouts, values},
};
pub use elvis_shared::LifeCycle;

/// A module which is typically glob imported from:
///
/// ```
/// use elvis::prelude::*;
/// ```
pub mod prelude {
    pub use elvis_derive::page;
    pub use elvis_shared::{LifeCycle, Node};
    #[cfg(feature = "web")]
    pub use elvis_web::{wasm_bindgen_re_exports::*, Page};
}

// elvis platform features
/// Browser data bridge, as default feature.
#[cfg(feature = "web")]
#[path = "platforms/web/mod.rs"]
mod web;
