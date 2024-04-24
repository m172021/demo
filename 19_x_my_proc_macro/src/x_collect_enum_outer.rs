use super::*;
use proc_macro2::Ident;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, DataEnum, DataUnion, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, ImplGenerics, TypeGenerics, WhereClause};

#[test]
pub fn test_00() {}

pub struct Collect_Enum_Outer {
    pub outer_write_js_code: Vec<proc_macro2::TokenStream>,
    pub outer_read_js_code: Vec<proc_macro2::TokenStream>,
    pub outer_write_bin_code: Vec<proc_macro2::TokenStream>,
    pub outer_read_bin_code: Vec<proc_macro2::TokenStream>,
}

impl Collect_Enum_Outer {
    pub fn new() -> Collect_Enum_Outer {
        Collect_Enum_Outer {
            outer_write_js_code: vec![],
            outer_read_js_code: vec![],
            outer_write_bin_code: vec![],
            outer_read_bin_code: vec![],
        }
    }

    pub fn gen_unit(
        outer_Ident: &proc_macro2::Ident,
        ident: &proc_macro2::Ident,
        idx_token: &proc_macro2::TokenStream,
    ) -> (
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
    ) {
        (
            quote! {
                #outer_Ident::#ident => { #idx_token.write_to_js(writer, /* abs, */ non_transfers, transfers )?; Ok(()) }
            },
            quote! {
                #idx_token => { Ok(#outer_Ident::#ident) }
            },
            quote! {
                #outer_Ident::#ident => { #idx_token.write_to_buf(writer)?; Ok(()) }
            },
            quote! {
                #idx_token => { Ok(#outer_Ident::#ident) }
            },
        )
    }

    pub fn output(
        self,
        label: &proc_macro2::TokenStream,
        outer_Ident: &proc_macro2::Ident,
        impl_generics: &ImplGenerics,
        ty_generics: &TypeGenerics,
        where_clause: &Option<&WhereClause>,
    ) -> proc_macro2::TokenStream {
        let Collect_Enum_Outer {
            outer_write_js_code,
            outer_read_js_code,
            outer_write_bin_code,
            outer_read_bin_code,
        } = self;

        quote! {

            impl #impl_generics Bintf_T for #outer_Ident #ty_generics #where_clause {
                fn write_to_js(&self, writer: &mut dyn std::io::Write,
                    non_transfers: &mut std::collections::VecDeque<wasm_bindgen::JsValue>,
                    transfers: &mut std::collections::VecDeque<wasm_bindgen::JsValue>,
                ) -> Result<(), Bintf_Err> {
                    match &self {
                        #(#outer_write_js_code)*
                    }
                }


                fn read_from_js(reader: &mut dyn std::io::Read,
                    non_transfers: &mut std::collections::VecDeque<wasm_bindgen::JsValue>,
                    transfers: &mut std::collections::VecDeque<wasm_bindgen::JsValue>,
                ) -> Result<Self, Bintf_Err> where Self: Sized {
                    let t: #label = <#label as Bintf_T>::read_from_js(reader, /* abs, */ non_transfers, transfers)?;
                    match t {
                        #(#outer_read_js_code)*
                        _ => { Err(Bintf_Err::Illegal_Enum) }
                    }
                }


                fn write_to_buf(&self, writer: &mut dyn std::io::Write) -> Result<(), Bintf_Err> {
                    match &self {
                        #(#outer_write_bin_code)*
                    }
                }


                fn read_from_buf(reader: &mut dyn std::io::Read) -> Result<Self, Bintf_Err> where Self: Sized {
                    let t: #label = <#label as Bintf_T>::read_from_buf(reader)?;
                    match t {
                        #(#outer_read_bin_code)*
                        _ => { Err(Bintf_Err::Illegal_Enum) }
                    }
                }
            }
        }
    }
}
