#![feature(try_trait)]
mod err;
mod parser;
mod tree;
mod values;
mod widgets;

pub use crate::{
    err::Error,
    parser::Parser,
    tree::Tree,
    values::unit::Unit,
    widgets::text::{Text, TextStyle},
};

// elvis platform features
#[cfg(feature = "web")]
pub mod web;
mod features {
    #[cfg(feature = "web")]
    pub use crate::web;
}
