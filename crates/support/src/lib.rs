//! Elvis derive macros
#![warn(missing_docs)]
extern crate proc_macro;

use proc_macro::TokenStream;
mod node;

/// `Into<Node>`
#[proc_macro_derive(IntoNode)]
pub fn node(input: TokenStream) -> TokenStream {
    node::parse(input)
}
