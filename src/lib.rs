#![feature(try_trait)]
// #[macro_use]
// extern crate elvis_macro;
mod attr;
mod element;
mod layout;
mod page;
mod text;
pub use attr::CSS;
pub use element::Element;
pub use layout::{Alignments, Layout};
pub use page::Page;
pub use text::{Text, TextStyle};

mod tree;
pub use tree::Tree;
mod err;
pub use err::Error;
