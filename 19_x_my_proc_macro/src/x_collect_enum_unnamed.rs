use super::*;
use proc_macro2::Ident;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, DataEnum, DataUnion, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, ImplGenerics, TypeGenerics, WhereClause};

#[test]
pub fn test_00() {}

pub struct Collect_Enum_Unnamed {
    pub parts: Vec<syn::Field>,
}

impl Collect_Enum_Unnamed {
    pub fn new0() -> Collect_Enum_Unnamed {
        Collect_Enum_Unnamed { parts: vec![] }
    }

    pub fn new(unnamed: Punctuated<syn::Field, syn::token::Comma>) -> Collect_Enum_Unnamed {
        let mut builder = Collect_Enum_Unnamed::new0();

        for part in unnamed.iter().cloned() {
            builder.parts.push(part.clone());
        }

        builder
    }

    pub fn output(
        &self,
        outer_Ident: &proc_macro2::Ident,
        ident: &proc_macro2::Ident,
        idx_token: &proc_macro2::TokenStream,
    ) -> (
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
    ) {
        let mut write_js_code = vec![];
        let mut read_js_code = vec![];
        let mut write_bin_code = vec![];
        let mut read_bin_code = vec![];
        let mut anon_js_vars = vec![];
        let mut anon_bin_vars = vec![];

        for (idx, part) in self.parts.iter().enumerate() {
            let ty = &part.ty;
            let new_Id = proc_macro2::Ident::new(&format!("x_{}", idx), proc_macro2::Span::call_site());

            write_js_code.push(quote! {
                #new_Id.write_to_js(writer, /* abs, */ non_transfers, transfers)?;
            });
            read_js_code.push(quote! {
                <#ty as Bintf_T>::read_from_js(reader, /* abs, */ non_transfers,  transfers)?
            });
            anon_js_vars.push(quote! { #new_Id });

            write_bin_code.push(quote! {
                #new_Id.write_to_buf(writer)?;
            });
            read_bin_code.push(quote! {
                <#ty as Bintf_T>::read_from_buf(reader)?
            });
            anon_bin_vars.push(quote! { #new_Id });
        }

        let write_js = quote! {
            #outer_Ident::#ident ( #(#anon_js_vars),* ) => {
                #idx_token.write_to_js(writer, non_transfers, transfers)?;
                #(#write_js_code);*;
                Ok(())
            }
        };
        let read_js = quote! {
            #idx_token => {
                Ok(#outer_Ident::#ident( #(#read_js_code),* ))
            }
        };
        let write_bin = quote! {
            #outer_Ident::#ident ( #(#anon_bin_vars),* ) => {
                #idx_token.write_to_buf(writer)?;
                #(#write_bin_code);*;
                Ok(())
            }
        };
        let read_bin = quote! {
        #idx_token => {
                Ok(#outer_Ident::#ident( #(#read_bin_code),* ))
            }
        };

        (write_js, read_js, write_bin, read_bin)
    }
}
