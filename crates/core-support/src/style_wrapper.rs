use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Add style field to struct
pub fn parse(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let ident = item.ident;

    TokenStream::from(quote! {
        impl StyleWrapper for #ident {
            fn wrap(self) -> Node {
                Into::<Node>::into(self.child).append_style(self.style)
            }
        }
    })
}
