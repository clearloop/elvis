//! Elvis derive macros
#![warn(missing_docs)]
extern crate proc_macro;

mod node;
mod page;

use proc_macro::TokenStream;

/// Basic elvis attr macro
#[proc_macro_attribute]
pub fn page(_attr: TokenStream, item: TokenStream) -> TokenStream {
    page::parse(_attr, item)
}

/// Derive `Into<Node>`
#[proc_macro_derive(IntoNode)]
pub fn node(input: TokenStream) -> TokenStream {
    node::parse(input)
}
