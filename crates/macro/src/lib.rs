extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;
// use syn::{parse_macro_input, AttributeArgs};

#[proc_macro_attribute]
pub fn elvis_element(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let ast = syn::parse(item).unwrap();
  // let attr = parse_macro_input!(attr as AttributeArgs);
  // let ident = &attr[0];
  // println!("{:#?}", &ident);
  // println!("{:#?}", &item);
  // Build the trait implementation
  impl_elvis_element_macro(&ast)
}

fn impl_elvis_element_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let gen = quote! {
    // #[wasm_bindgen]
    impl #name {
      // #[wasm_bidgen(getter)]
      fn ee(self) -> ElvisElement {
        self.el
      }
    }
  };

  gen.into()
}
