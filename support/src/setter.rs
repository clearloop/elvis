use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Field, Fields, Type};

/// Basic elvis attr macro
pub fn parse(input: TokenStream) -> TokenStream {
    const SKIP_FIELD: &str = "skip";
    let input = parse_macro_input!(input as DeriveInput);

    // Get fields without `skip` attribute
    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    }
    .iter()
    .filter(|f| {
        f.attrs
            .iter()
            .find(|a| a.path.is_ident(SKIP_FIELD))
            .is_none()
    })
    .collect::<Vec<&Field>>();

    let struct_name = &input.ident;
    let struct_new_doc = format!("new {}", struct_name);

    let mut field_type: Vec<&Type> = vec![];
    let mut field_name: Vec<&Option<Ident>> = vec![];
    let mut field_doc: Vec<String> = vec![];
    let mut optional_field_type: Vec<&Type> = vec![];
    let mut optional_field_name: Vec<&Option<Ident>> = vec![];
    let mut optional_field_doc: Vec<String> = vec![];

    fields.iter().for_each(|field| {
        if let Some(ty) = super::helper::extract_type_from_option(&field.ty) {
            optional_field_type.push(&ty);
            optional_field_name.push(&field.ident);
            optional_field_doc.push(format!("Set {}", field.ident.clone().unwrap().to_string()));
        } else {
            field_type.push(&field.ty);
            field_name.push(&field.ident);
            field_doc.push(format!("Set {}", field.ident.clone().unwrap().to_string()));
        }
    });

    TokenStream::from(quote! {
        impl #struct_name {
            #[doc = #struct_new_doc]
            pub fn new() -> #struct_name {
                #struct_name::default()
            }

            #(
                #[doc = #field_doc]
                pub fn #field_name(mut self, v: impl Into<#field_type>) -> Self {
                    self.#field_name = v.into();
                    self
                }
            )*

            #(
                #[doc = #optional_field_doc]
                pub fn #optional_field_name(mut self, v: impl Into<#optional_field_type>) -> Self {
                    self.#optional_field_name = Some(v.into());
                    self
                }
            )*
        }
    })
}
