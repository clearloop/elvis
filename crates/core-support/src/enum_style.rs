use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/// Basic elvis attr macro
pub fn parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let vars = match &input.data {
        Data::Enum(d) => d.variants.iter().map(|var| var.ident.clone()),
        _ => panic!("expected a enum with named variants"),
    };
    let enum_name = &input.ident;

    TokenStream::from(quote! {
        impl ToString for #enum_name {
            fn to_string(&self) -> String {
                match self {
                    #(
                        #enum_name::#vars => stringify!(#vars).to_ascii_lowercase(),
                    )*
                }
            }
        }
    })
}
