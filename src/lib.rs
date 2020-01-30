#![feature(try_trait)]
mod err;
mod serde;
mod tree;
mod values;
mod widgets;

pub use crate::{
    err::Error,
    serde::Serde,
    tree::Tree,
    values::{color::Colors, unit::Unit},
    widgets::text::{Text, TextStyle},
};

// elvis platform features
#[cfg(feature = "web")]
pub mod web;
mod features {
    #[cfg(feature = "web")]
    pub use crate::web;
}
