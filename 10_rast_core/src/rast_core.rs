use super::*;

use proc_macro2::{Ident, TokenStream, TokenTree};
use std::collections::HashSet;
use syn::{Attribute, Fields, FnArg, GenericArgument, Pat, PathArguments, ReturnType, TraitItem, Type};
use syn::{Item, ItemEnum};

pub struct Rast_Util {}

pub enum Rast_Fields {
    Unit,
    Unnamed(Vec<Type>),
    Named(Vec<(proc_macro2::Ident, Type)>),
}

pub struct Rast_Struct {
    pub generics: syn::Generics,
    pub attrs: HashSet<String>,
    pub name: proc_macro2::Ident,
    pub fields: Rast_Fields,
}

pub struct Rast_Enum {
    pub generics: syn::Generics,
    pub attrs: HashSet<String>,
    pub name: proc_macro2::Ident,
    pub variants: Vec<(proc_macro2::Ident, Rast_Fields)>,
}

// ===========================

impl Rast_Util {
    pub fn ident_to_string(ty: &syn::Ident) -> String {
        let tokens = quote::quote! { #ty };
        tokens.to_string()
    }
}

impl Rast_Struct {
    pub fn new(s: &syn::ItemStruct) -> Rast_Struct {
        Rast_Struct {
            generics: s.generics.clone(),
            attrs: {
                let mut out = vec![];
                for x in s.attrs.clone().into_iter() {
                    if let Ok(y) = x.meta.require_list() {
                        Rast_File::get_idents_hlpr(&mut out, y.tokens.clone());
                    }
                }
                out.iter().map(|x| Rast_Util::ident_to_string(x)).collect::<HashSet<_>>()
            },
            name: s.ident.clone(),
            fields: Rast_Fields::new(&s.fields),
        }
    }
}

impl Rast_Enum {
    pub fn new(e: &syn::ItemEnum) -> Rast_Enum {
        Rast_Enum {
            generics: e.generics.clone(),
            attrs: {
                let mut out = vec![];
                for x in e.attrs.clone().into_iter() {
                    if let Ok(y) = x.meta.require_list() {
                        Rast_File::get_idents_hlpr(&mut out, y.tokens.clone());
                    }
                }
                out.iter().map(|x| Rast_Util::ident_to_string(x)).collect::<HashSet<_>>()
            },
            name: e.ident.clone(),
            variants: (e.variants.iter()).map(|x| (x.ident.clone(), Rast_Fields::new(&x.fields))).collect::<Vec<_>>(),
        }
    }
}

impl Rast_Util {
    fn type_to_string(ty: &Type) -> String {
        let tokens = quote::quote! { #ty };
        tokens.to_string()
    }
}

impl Rast_Fields {
    pub fn new(x: &Fields) -> Rast_Fields {
        match x {
            Fields::Named(x) => Rast_Fields::Named(x.named.iter().map(|x| (x.ident.clone().unwrap(), x.ty.clone())).collect::<Vec<_>>()),
            Fields::Unnamed(x) => Rast_Fields::Unnamed(x.unnamed.iter().map(|x| x.ty.clone()).collect::<Vec<_>>()),
            Fields::Unit => Rast_Fields::Unit,
        }
    }
}

pub struct Rast_File {
    pub structs: Vec<Rast_Struct>,
    pub enums: Vec<Rast_Enum>,
}

impl Rast_File {
    fn get_idents_hlpr(out: &mut Vec<Ident>, x: TokenStream) {
        for y in x {
            match y {
                TokenTree::Group(g) => Self::get_idents_hlpr(out, g.stream()),
                TokenTree::Ident(i) => {
                    out.push(i);
                }
                TokenTree::Punct(_) => {}
                TokenTree::Literal(_) => {}
            }
        }
    }

    pub fn new_from_file(fname: &str) -> Rast_File {
        let s = std::fs::read_to_string(fname).unwrap();
        let tokens: proc_macro2::TokenStream = s.parse().unwrap();
        let file: syn::File = syn::parse2(tokens).unwrap();
        let mut parts = Rast_File {
            structs: vec![],
            enums: vec![],
        };
        parts.parse_stream(file);
        parts
    }

    pub fn parse_stream(&mut self, file: syn::File) -> Option<()> {
        for item in file.items.into_iter() {
            match item {
                Item::Struct(x) => self.structs.push(Rast_Struct::new(&x)),
                Item::Enum(x) => self.enums.push(Rast_Enum::new(&x)),
                _ => {}
            }
        }
        Some(())
    }
}
