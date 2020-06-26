//! This library implements Elvis in browser, most of the contents are interfaces.
//!
//! You can rust the web with this crate, and...no javascript usages in this doc mainly because rust-doc can not highlight `javascript` code, we strongly recommend you reading [The Book][1] if you wondering how to "calling elvis".
//!
//! [1]: https://clearloop.github.io/elvis
mod style;
mod widgets;

pub use crate::{style::StyleSheet, widgets::Widget};
