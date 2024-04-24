use super::*;

struct Util {}
impl Util {
    pub fn inject_my_trait(generics: &mut Generics) {
        for x in &mut generics.params {
            match x {
                GenericParam::Type(t) => {
                    let mut x = Punctuated::new();
                    x.push(PathSegment {
                        ident: proc_macro2::Ident::new("Bintf_T", proc_macro2::Span::call_site()),
                        arguments: Default::default(),
                    });
                    t.bounds.push(TypeParamBound::Trait(TraitBound {
                        paren_token: None,
                        modifier: TraitBoundModifier::None,
                        lifetimes: None,
                        path: Path {
                            leading_colon: None,
                            segments: x,
                        },
                    }));
                }
                GenericParam::Lifetime(_) => {}
                GenericParam::Const(_) => {}
            }
        }
    }
}

pub struct X_Bintf {}

impl X_Bintf {
    pub fn proc_x_bintf(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
        let DeriveInput {
            ident: outer_Ident,
            data,
            mut generics,
            ..
        } = parse_macro_input!(input);

        Util::inject_my_trait(&mut generics);

        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let output = match data {
            syn::Data::Struct(s) => match s.fields {
                syn::Fields::Named(FieldsNamed { named, .. }) => {
                    let builder = Collect_Struct_Named::new(named);
                    builder.output(&outer_Ident, &impl_generics, &ty_generics, &where_clause)
                }

                syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                    let builder = Collect_Struct_Unnamed::new(unnamed);
                    builder.output(&outer_Ident, &impl_generics, &ty_generics, &where_clause)
                }

                syn::Fields::Unit => {
                    let builder = Collect_Struct_Named::new0();
                    builder.output(&outer_Ident, &impl_generics, &ty_generics, &where_clause)
                }
            },

            syn::Data::Enum(DataEnum { variants, .. }) => {
                let mut builder = Collect_Enum_Outer::new();

                let variants_len = variants.len();
                let label = if variants_len >= 255 {
                    quote! { u16 }
                } else {
                    quote! { u8 }
                };

                for (idx, part) in variants.iter().cloned().enumerate() {
                    let idx_token = if variants_len >= 255 {
                        let idx_u16 = idx as u16;
                        quote! { #idx_u16 }
                    } else {
                        let idx_u8 = idx as u8;
                        quote! { #idx_u8 }
                    };

                    let ident = part.ident;
                    let fields = part.fields;
                    let (write_js_code, read_js_code, write_bin_code, read_bin_code) = match fields {
                        Fields::Named(FieldsNamed { named, .. }) => {
                            let builder = Collect_Enum_Named::new(named);
                            builder.output(&outer_Ident, &ident, &idx_token)
                        }
                        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                            let builder = Collect_Enum_Unnamed::new(unnamed);
                            builder.output(&outer_Ident, &ident, &idx_token)
                        }
                        Fields::Unit => Collect_Enum_Outer::gen_unit(&outer_Ident, &ident, &idx_token),
                    };

                    builder.outer_write_js_code.push(write_js_code);
                    builder.outer_read_js_code.push(read_js_code);

                    builder.outer_write_bin_code.push(write_bin_code);
                    builder.outer_read_bin_code.push(read_bin_code);
                }

                builder.output(&label, &outer_Ident, &impl_generics, &ty_generics, &where_clause)
            }
            syn::Data::Union(DataUnion {
                fields: FieldsNamed { .. }, ..
            }) => {
                todo!("error in x_bintf")
            }
        };
        output.into()
    }
}
