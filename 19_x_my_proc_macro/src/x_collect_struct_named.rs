// use proc_macro::{self, TokenStream};
use super::*;
use proc_macro2::Ident;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, DataEnum, DataUnion, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, ImplGenerics, TypeGenerics, WhereClause};

#[test]
pub fn test_00() {}

pub struct Collect_Struct_Named {
    pub parts: Vec<syn::Field>,
}

impl Collect_Struct_Named {
    pub fn new0() -> Collect_Struct_Named {
        Collect_Struct_Named { parts: vec![] }
    }

    pub fn new(named: Punctuated<syn::Field, syn::token::Comma>) -> Collect_Struct_Named {
        let mut builder = Collect_Struct_Named::new0();

        for part in named.iter().cloned() {
            builder.parts.push(part.clone());
        }

        builder
    }

    pub fn output(
        self,
        outer_Ident: &proc_macro2::Ident,
        impl_generics: &ImplGenerics,
        ty_generics: &TypeGenerics,
        where_clause: &Option<&WhereClause>,
    ) -> proc_macro2::TokenStream {
        let Collect_Struct_Named { parts } = self;

        let mut write_js_code = vec![];
        let mut read_js_code = vec![];
        let mut write_bin_code = vec![];
        let mut read_bin_code = vec![];

        for part in parts.iter() {
            let x = part.clone().ident.unwrap();
            let ty = &part.ty;

            write_js_code.push(quote! {
                self.#x.write_to_js(writer, /* abs, */ non_transfers, transfers)?;
            });

            read_js_code.push(quote! {
                #x: <#ty as Bintf_T>::read_from_js(reader, /* abs, */ non_transfers, transfers)?,
            });

            write_bin_code.push(quote! {
                self.#x.write_to_buf(writer)?;
            });

            read_bin_code.push(quote! {
                #x: <#ty as Bintf_T>::read_from_buf(reader)?,
            });
        }

        quote! {
            impl #impl_generics Bintf_T for #outer_Ident #ty_generics #where_clause {
                fn write_to_js(
                        &self,
                        writer: &mut dyn std::io::Write,
                        non_transfers: &mut std::collections::VecDeque<wasm_bindgen::JsValue>,
                        transfers: &mut std::collections::VecDeque<wasm_bindgen::JsValue>,

                    )
                    -> Result<(), Bintf_Err> {
                    #(#write_js_code);*;
                    Ok(())
                }

                fn read_from_js(
                        reader: &mut dyn std::io::Read,
                        non_transfers: &mut std::collections::VecDeque<wasm_bindgen::JsValue>,
                        transfers: &mut std::collections::VecDeque<wasm_bindgen::JsValue>,
                    )
                    -> Result<Self, Bintf_Err> where Self: Sized {
                    Ok( #outer_Ident {
                            #(#read_js_code)*
                        } ) }

                fn write_to_buf(
                        &self,
                        writer: &mut dyn std::io::Write)
                    -> Result<(), Bintf_Err> {
                    #(#write_bin_code);*;
                    Ok(())
                }

                fn read_from_buf(
                        reader: &mut dyn std::io::Read)
                    -> Result<Self, Bintf_Err> where Self: Sized {
                    Ok( #outer_Ident { #(#read_bin_code)* } )
                }
            }
        }
    }
}
