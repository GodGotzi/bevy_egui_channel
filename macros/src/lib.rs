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
        use bevy_egui_events::events::*;

        #[allow(dead_code)]
        #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
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

        impl EventCollection<#enum_name> for #type_enum {
            fn event_eq_type(&self, other: #enum_name) -> bool {
                match (self, other) {
                    #((#type_enum::#variants, #enum_name::#variants(_)) => true,) *
                    _ => false
                }
            }
        }

        impl Opposite<#enum_name> for #type_enum {
            fn opposite(&self) -> #enum_name {
                match self {
                    #(#type_enum::#variants => #enum_name::#variants(Default::default()),)*
                }
            }
        }

        impl Opposite<#type_enum> for #enum_name {
            fn opposite(&self) -> #type_enum {
                match self {
                    #(#enum_name::#variants(_) => #type_enum::#variants,)*
                }
            }
        }

        impl event_traits::EnumVec<#type_enum> for #type_enum {
            fn as_vec() -> Vec<#type_enum> {
                vec![#(#type_enum::#variants,)*]
            }
        }

        impl event_traits::EnumVec<#enum_name> for #enum_name {
            fn as_vec() -> Vec<#enum_name> {
                vec![#(#enum_name::#variants(Default::default()),)*]
            }
        }

    };

    TokenStream::from(expanded)
}