extern crate proc_macro;


use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse, Ident, DeriveInput};

#[proc_macro_derive(EventCollection)]
pub fn event_collection_derive(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse(input).unwrap();
    // Get the name of the enum
    let enum_name = &derive_input.ident;

    let variants: Vec<Ident> = match derive_input.data {
        syn::Data::Struct(_) => panic!("Structs are not supported with TypeEq"),
        syn::Data::Enum(data) => data.variants.into_iter().map(|f| f.ident).collect(),
        syn::Data::Union(_) => panic!("Union are not supported with TypeEq"),
    };

    let type_enum = Ident::new(format!("{}Type", enum_name.to_string()).as_str(), Span::call_site());

    // Generate the implementation for `TypeEq` trait
    let expanded = quote! {

        #[allow(dead_code)]
        #[derive(Debug, Copy, Clone)]
        pub enum #type_enum {
            #(#variants,)*
        }

        impl EventCollection<#type_enum> for #enum_name {
            fn event_eq_type(&self, other: #type_enum) -> bool {
                match (self, other) {
                    #((#enum_name::#variants(_), #type_enum::#variants) => true,) *
                    _ => false
                }
            }
        }

    };

    TokenStream::from(expanded)
}