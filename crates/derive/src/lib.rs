//! Elvis derive macros
#![warn(missing_docs)]
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Basic elvis attr macro
#[proc_macro_attribute]
pub fn page(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    let ident = item.ident.clone();
    let fnn = Ident::new(&ident.to_string().to_lowercase(), Span::mixed_site());

    let expanded = quote! {
        #item

        /// Run APP
        #[wasm_bindgen]
        pub fn #fnn() {
            let mut page = Page::from(#ident.create());
            page.calling().unwrap();
        }
    };

    TokenStream::from(expanded)
}
