use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

/// From ref into Node
pub fn ref_into_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    TokenStream::from(quote! {
        impl Into<Node> for #struct_name {
            fn into(self) -> Node {
                let s = &self;
                s.into()
            }
        }
    })
}

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

    let ref_into = quote! {
        impl Into<Node> for #struct_name {
            fn into(self) -> Node {
                let s = &self;
                s.into()
            }
        }
    };

    TokenStream::from(
        if fields.contains(&"child".to_string()) && fields.contains(&"style".to_string()) {
            quote! {
                impl Into<Node> for &#struct_name {
                    fn into(self) -> Node {
                        Node::default()
                            .children(vec![self.child.clone()])
                            .class(&mut vec![Class::Flex])
                            .style(self.style.clone())
                    }
                }

                #ref_into
            }
        } else if fields.contains(&"child".to_string()) && !fields.contains(&"style".to_string()) {
            quote! {
                impl Into<Node> for &#struct_name {
                    fn into(self) -> Node {
                        Node::default()
                            .children(vec![self.child.clone()])
                            .class(&mut vec![Class::Flex])
                    }
                }

                #ref_into
            }
        } else if fields.contains(&"style".to_string()) {
            quote! {
                impl Into<Node> for &#struct_name {
                    fn into(self) -> Node {
                        Node::default()
                            .children(self.children.iter().map(|c| c.clone()).collect())
                            .class(&mut vec![
                                Class::Flex,
                                Class::from(stringify!($widget).to_lowercase().as_str()),
                            ])
                            .style(self.style.clone())
                    }
                }

                #ref_into
            }
        } else {
            quote! {
                impl Into<Node> for &#struct_name {
                    fn into(self) -> Node {
                        Node::default()
                            .children(self.children.iter().map(|c| c.clone()).collect())
                            .class(&mut vec![
                                Class::Flex,
                                Class::from(stringify!($widget).to_lowercase().as_str()),
                            ])
                    }
                }

                #ref_into
            }
        },
    )
}
