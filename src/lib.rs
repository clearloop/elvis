#![feature(try_trait)]
#![feature(box_syntax)]
#![feature(vec_remove_item)]
mod bridge;
mod err;
mod layout;
mod life;
mod serde;
mod tree;
mod values;
mod widgets;

pub use crate::{
    err::Error,
    layout::*,
    life::{Life, LifeCycle},
    serde::Serde,
    tree::Tree,
    values::{color::Colors, layout::*, unit::Unit},
    widgets::{Image, ImageSrc, Text, TextStyle},
};

// elvis platform features
#[cfg(feature = "web")]
pub mod web;
mod features {
    #[cfg(feature = "web")]
    pub use crate::web;
}
