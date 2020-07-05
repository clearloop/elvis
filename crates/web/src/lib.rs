//! This library implements Elvis in browser, most of the contents are interfaces.
//!
//! You can rust the web with this crate, and...no javascript usages in this doc mainly because rust-doc can not highlight `javascript` code, we strongly recommend you reading [The Book][1] if you wondering how to "calling elvis".
//!
//! [1]: https://clearloop.github.io/elvis
#![warn(missing_docs)]
mod style;
mod widget;

pub use self::widget::Widget;
use style::StyleSheet;

/// Re-exports wasm-bindgen
pub mod wasm_bindgen_re_exports {
    pub use wasm_bindgen;
    pub use wasm_bindgen::prelude::*;
}
