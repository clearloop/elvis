extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ElvisElement)]
pub fn elvis_element_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();

  // Build the trait implementation
  impl_elvis_element_macro(&ast)
}

fn impl_elvis_element_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;

  let gen = quote! {
      impl #name {
          fn ee(self) -> ElvisElement {
              self.el
          }
      }
  };
  gen.into()
}
