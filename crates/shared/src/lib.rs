//! Elvis shared library
#![warn(missing_docs)]
mod err;
mod life;
mod serde;
mod tree;

pub use self::{
    err::Error,
    life::LifeCycle,
    serde::Serde,
    tree::{FnBox, Node, State},
};
