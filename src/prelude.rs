//! A module which is typically glob imported from:
//!
//! ```
//! use elvis::prelude::*;
//! ```
pub use elvis_core::{LifeCycle, Node};
pub use elvis_derive::page;
#[cfg(feature = "web")]
pub use elvis_web::{wasm_bindgen_re_exports::*, Page};
