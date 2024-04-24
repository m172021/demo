// use proc_macro::{self, TokenStream};

use super::*;

use proc_macro2::Ident;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, DataEnum, DataUnion, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, ImplGenerics, TypeGenerics, WhereClause};

#[test]
pub fn test_00() {}

pub struct Collect_Enum_Named {
    pub parts: Vec<syn::Field>,
}

impl Collect_Enum_Named {
    pub fn new0() -> Collect_Enum_Named {
        Collect_Enum_Named { parts: vec![] }
    }

    pub fn new(named: Punctuated<syn::Field, syn::token::Comma>) -> Collect_Enum_Named {
        let mut builder = Collect_Enum_Named::new0();

        for part in named.iter().cloned() {
            builder.parts.push(part);
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
        let mut args = vec![];

        for part in self.parts.iter() {
            let x = part.clone().ident.unwrap();
            let ty = &part.ty;

            args.push(quote! { #x });
            write_js_code.push(quote! {
                #x.write_to_js(writer, /* abs, */ non_transfers, transfers )?;
            });

            read_js_code.push(quote! {
                #x: <#ty as Bintf_T>::read_from_js(reader, /* abs, */non_transfers, transfers)?,
            });

            write_bin_code.push(quote! {
                #x.write_to_buf(writer)?;
            });

            read_bin_code.push(quote! {
                #x: <#ty as Bintf_T>::read_from_buf(reader)?,
            });
        }

        let write_js = quote! {
        #outer_Ident::#ident { #(#args),* } => {
            #idx_token.write_to_js(writer, /* abs, */ non_transfers, transfers)?;
            #(#write_js_code);*;
            Ok(()) } };

        let read_js = quote! {
        #idx_token => {
        Ok( #outer_Ident::#ident {
                #(#read_js_code)* } ) } };

        let write_bin = quote! {
        #outer_Ident::#ident { #(#args),* } => {
            #idx_token.write_to_buf(writer)?;
            #(#write_bin_code);*;
            Ok(()) } };

        let read_bin = quote! {
        #idx_token => {
        Ok( #outer_Ident::#ident {
                #(#read_bin_code)* } ) } };

        (write_js, read_js, write_bin, read_bin)
    }
}
