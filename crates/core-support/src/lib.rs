//! Elvis derive macros
#![warn(missing_docs)]
extern crate proc_macro;

use proc_macro::TokenStream;
mod enum_style;
mod setter;

/// Basic elvis attr macro
#[proc_macro_derive(Setter)]
pub fn setter(input: TokenStream) -> TokenStream {
    setter::parse(input)
}

/// Basic elvis enum style
#[proc_macro_derive(EnumStyle)]
pub fn enum_style(input: TokenStream) -> TokenStream {
    enum_style::parse(input)
}
