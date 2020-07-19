//! This library implements Elvis in browser, most of the contents are interfaces.
//!
//! You can rust the web with this crate, and...no javascript usages in this doc mainly because rust-doc can not highlight `javascript` code, we strongly recommend you reading [The Book][1] if you wondering how to "calling elvis".
//!
//! [1]: https://clearloop.github.io/elvis
#![warn(missing_docs)]
mod driver;
pub mod event;
mod gesture;
mod node;
mod page;
mod router;
mod style;

use self::style::StyleSheet;
pub use self::{driver::WebDriver, page::Page, router::Router};

/// Re-exports wasm-bindgen
pub mod wasm_bindgen_re_exports {
    pub use wasm_bindgen;
    pub use wasm_bindgen::prelude::*;
}
