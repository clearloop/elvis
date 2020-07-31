//! Elvis derive macros
#![warn(missing_docs)]
extern crate proc_macro;

use self::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

/// Basic elvis attr macro
#[proc_macro_derive(Setter)]
pub fn setter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let field_name = fields.iter().map(|field| &field.ident);
    let field_type = fields.iter().map(|field| &field.ty);
    let field_doc = fields
        .iter()
        .map(|field| format!("Set {}", field.ident.clone().unwrap().to_string()));
    let struct_name = &input.ident;
    let struct_new_doc = format!("new {}", struct_name);

    TokenStream::from(quote! {
        impl #struct_name {
            #[doc = #struct_new_doc]
            pub fn new() -> #struct_name {
                #struct_name::default()
            }

            #(
                #[doc = #field_doc]
                pub fn #field_name(mut self, v: #field_type) -> Self {
                    self.#field_name = v;
                    self
                }
            )*
        }
    })
}
