use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

/// Single Child Widget
pub fn parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    }
    .iter()
    .map(|field| field.ident.clone().unwrap().to_string())
    .collect::<Vec<String>>();

    TokenStream::from(
        if fields.contains(&"child".to_string()) && fields.contains(&"style".to_string()) {
            quote! {
                impl Into<Node> for #struct_name {
                    fn into(self) -> Node {
                        Node::default()
                            .children(vec![self.child])
                            .class(vec![Class::Flex])
                            .style(self.style.clone())
                    }
                }
            }
        } else if fields.contains(&"child".to_string()) && !fields.contains(&"style".to_string()) {
            quote! {
                impl Into<Node> for #struct_name {
                    fn into(self) -> Node {
                        Node::default()
                            .children(vec![self.child])
                            .class(vec![Class::Flex])
                    }
                }
            }
        } else if fields.contains(&"style".to_string()) {
            quote! {
                impl Into<Node> for #struct_name {
                    fn into(self) -> Node {
                        Node::default()
                            .children(self.children)
                            .class(vec![
                                Class::Flex,
                                Class::from(stringify!($widget).to_lowercase().as_str()),
                            ])
                            .style(self.style)
                    }
                }
            }
        } else {
            quote! {
                impl Into<Node> for #struct_name {
                    fn into(self) -> Node {
                        Node::default()
                            .children(self.children)
                            .class(vec![
                                Class::Flex,
                                Class::from(stringify!($widget).to_lowercase().as_str()),
                            ])
                    }
                }
            }
        },
    )
}
