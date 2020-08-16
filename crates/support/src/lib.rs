//! Elvis derive macros
#![warn(missing_docs)]
extern crate proc_macro;

use proc_macro::TokenStream;
mod enum_style;
mod helper;
mod setter;
mod style_wrapper;

/// Basic elvis attr macro
#[proc_macro_derive(Setter, attributes(skip))]
pub fn setter(input: TokenStream) -> TokenStream {
    setter::parse(input)
}

/// Basic elvis enum style
#[proc_macro_derive(EnumStyle)]
pub fn enum_style(input: TokenStream) -> TokenStream {
    enum_style::parse(input)
}

/// Basic elvis enum style
#[proc_macro_derive(Wrapper)]
pub fn style_wrapper(input: TokenStream) -> TokenStream {
    style_wrapper::parse(input)
}
