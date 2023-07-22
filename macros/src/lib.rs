extern crate proc_macro;

use proc_macro::TokenStream;
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

    // Generate the implementation for `TypeEq` trait
    let expanded = quote! {
        use event_traits::TypeEq;

        impl TypeEq<#enum_name> for #enum_name {
            fn type_eq(&self, other: #enum_name) -> bool {
                match (self, other) {
                    #((#enum_name::#variants(_), #enum_name::#variants(_)) => true,) *
                    _ => false
                } 
            }
        }
    };

    TokenStream::from(expanded)
}

macro_rules! generate_events {
    () => {
        #[derive(Clone, Debug)]
        pub enum Item {
            ToolbarWidth(TransferValue<f32>),
            SettingsWidth(TransferValue<f32>),
            LayerValue(TransferValue<u32>),
            TimeValue(TransferValue<f32>),
        }

        impl TypeEq 
    };
}