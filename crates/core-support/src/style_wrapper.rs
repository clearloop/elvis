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
                let node = Into::<Node>::into(self);
                let mut child = node.children[0].borrow().clone();
                child.append_style(node.style)
            }
        }
    })
}
