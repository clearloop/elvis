extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn elvis(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    let ident = item.ident.clone();

    let expanded = quote! {
        #item

        impl #ident {
            pub fn hello() {
                println!("world");
            }
        }
    };
    println!("{:?}", &expanded);
    TokenStream::from(expanded)
}
