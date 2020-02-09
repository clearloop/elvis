#![feature(try_trait)]
#![feature(box_syntax)]
#![feature(vec_remove_item)]
mod bridge;
mod err;
mod life;
mod serde;
mod tree;
mod values;
mod widgets;

pub use crate::{
    err::Error,
    life::{Life, LifeCycle},
    serde::Serde,
    tree::Tree,
    values::{color::Colors, unit::Unit},
    widgets::{Image, Text, TextStyle},
};

// elvis platform features
#[cfg(feature = "web")]
pub mod web;
mod features {
    #[cfg(feature = "web")]
    pub use crate::web;
}
