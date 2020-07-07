//! Elvis shared library
#![warn(missing_docs)]
mod err;
mod serde;
mod tree;

pub use self::{
    err::Error,
    serde::Serde,
    tree::{FnBox, LifeCycle, Node, State},
};
