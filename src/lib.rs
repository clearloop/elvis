#![feature(try_trait)]
mod err;
mod tree;
pub use tree::{Tree, TreeParser};

// elvis platform features
#[cfg(feature = "web")]
pub mod web;
mod features {
    #[cfg(feature = "web")]
    pub use crate::web;
}
