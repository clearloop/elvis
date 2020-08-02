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

/// Ref into `Node`
#[proc_macro_derive(RefIntoNode)]
pub fn ref_into_node(input: TokenStream) -> TokenStream {
    node::ref_into_node(input)
}
