#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use macro_include::*;

use proc_macro::Span;
use proc_macro2::Ident;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, DataEnum, DataUnion, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, GenericParam, Generics, ImplGenerics, Path, PathSegment,
    TraitBound, TraitBoundModifier, TypeGenerics, TypeParam, TypeParamBound, Variant, WhereClause,
};

mod x_collect_enum_named;
mod x_collect_enum_outer;
mod x_collect_enum_unnamed;
mod x_collect_struct_named;
mod x_collect_struct_unnamed;

use x_collect_enum_named::Collect_Enum_Named;
use x_collect_enum_outer::Collect_Enum_Outer;
use x_collect_enum_unnamed::Collect_Enum_Unnamed;
use x_collect_struct_named::Collect_Struct_Named;
use x_collect_struct_unnamed::Collect_Struct_Unnamed;

mod enum_usize;
mod tui_val;
mod x_bintf;

#[proc_macro_derive(Fake_X_Partial)]
pub fn proc_x_partial(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let t = quote! {};
    t.into()
}

#[proc_macro_derive(Fake_X_Draw_Config)]
pub fn proc_x_draw_config(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let t = quote! {};
    t.into()
}

#[proc_macro_derive(Fake_Rr_Val)]
pub fn proc_x_tui_draw(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let t = quote! {};
    t.into()
}

#[proc_macro_derive(Bintf)]
pub fn proc_x_bintf(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    x_bintf::X_Bintf::proc_x_bintf(input)
}

#[proc_macro_derive(Enum_Usize)]
pub fn proc_enum_usize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    enum_usize::Enum_USize::proc_enum_usize(input)
}

#[proc_macro_derive(Rr_Val)]
pub fn proc_tui_val(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    tui_val::Rr_Val::proc_rr_val(input)
}
