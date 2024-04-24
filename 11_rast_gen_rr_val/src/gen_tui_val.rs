use super::*;
use proc_macro2::TokenStream;
use syn::GenericParam;

pub struct Rast_Gen_Rr_Val {}

impl Rast_Gen_Rr_Val {
    pub fn gen_fields_config(f: &Rast_Fields) -> TokenStream {
        match f {
            Rast_Fields::Unnamed(x) => {
                let len = x.len();
                quote::quote! {
                    Rr_Desc_Fields::Unnamed( #len )
                }
            }
            Rast_Fields::Named(x) => {
                let mut names = vec![];
                for (ident, _) in x.iter() {
                    names.push(proc_macro2::Literal::string(&ident.to_string()));
                }
                quote::quote! {
                    Rr_Desc_Fields::Named( &[ #(#names),* ] )
                }
            }
            Rast_Fields::Unit => {
                quote::quote! { Rr_Desc_Fields::Unit }
            }
        }
    }

    pub fn gen_struct(x: &Rast_Struct) -> TokenStream {
        let ident = syn::Ident::new(&x.name.to_string(), x.name.span());

        let field_config = Self::gen_fields_config(&x.fields);

        let aux = quote::quote! {
            fn rr_get_config(&self) -> Rr_Desc_Struct {
                Rr_Desc_Struct { fields: #field_config }
            }

        };

        let (gen_impl, gen_ty, _c) = &x.generics.split_for_impl();
        let w_clauses = {
            let mut out = vec![];
            for x in x.generics.params.iter() {
                match x {
                    GenericParam::Lifetime(_) => {}
                    GenericParam::Type(t) => {
                        let nm = &t.ident;
                        out.push(quote::quote! { #nm : Rr_Any_T });
                    }
                    GenericParam::Const(_) => {}
                }
            }
            out
        };

        let struct_any = quote::quote! {
            impl #gen_impl Rr_Any_T for #ident #gen_ty where #(#w_clauses),* {
                fn rr_to_any_ref<'a>(&'a self) -> Rr_Any_Ref<'a> {
                    Rr_Any_Ref::Struct(self as _)
                }

                fn rr_to_mut_any_ref<'a>(&'a mut self) -> Rr_Mut_Any_Ref<'a> {
                    Rr_Mut_Any_Ref::Struct(self as _)
                }
            }
        };

        let (arms, arms_mut) = {
            let mut arms = vec![];
            let mut arms_mut = vec![];

            match &x.fields {
                Rast_Fields::Named(x) => {
                    for (i, (ident, _ty)) in x.iter().enumerate() {
                        arms.push(quote::quote! { #i => Ok( (& self.#ident).rr_to_any_ref() ), });
                        arms_mut.push(quote::quote! { #i => Ok( (& mut self.#ident).rr_to_mut_any_ref() ), });
                    }
                }
                Rast_Fields::Unnamed(x) => {
                    for (i, _ty) in x.iter().enumerate() {
                        let idx = proc_macro2::Literal::usize_unsuffixed(i);
                        arms.push(quote::quote! { #i => Ok( (& self.#idx).rr_to_any_ref() ), });
                        arms_mut.push(quote::quote! { #i => Ok( (& mut self.#idx).rr_to_mut_any_ref() ), });
                    }
                }
                Rast_Fields::Unit => {}
            }
            (arms, arms_mut)
        };

        quote::quote! {

            #struct_any

            impl #gen_impl Rr_Struct_T for #ident #gen_ty  where #(#w_clauses),* {

                fn rr_get<'a>(&'a self, id: usize) -> Result<Rr_Any_Ref<'a>, Rr_Err> {
                    match id {
                        #(#arms)*
                        _ => { Err(Rr_Err::Get_Oob { val: id }) }
                    }
                }

                fn rr_get_mut<'a>(&'a mut self, id: usize) -> Result<Rr_Mut_Any_Ref<'a>, Rr_Err> {
                    match id {
                        #(#arms_mut)*
                        _ => { Err(Rr_Err::Get_Oob { val: id }) }
                    }
                }

                #aux
            }
        }
    }

    pub fn gen_enum(x: &Rast_Enum) -> TokenStream {
        let name_enum = syn::Ident::new(&x.name.to_string(), x.name.span());

        let get_config = {
            let mut config = vec![];
            for (_i, (ident, fields)) in x.variants.iter().enumerate() {
                let f = Self::gen_fields_config(fields);
                let ident = proc_macro2::Literal::string(&ident.to_string());
                config.push(quote::quote! { ( #ident, #f ) });
            }
            quote::quote! {
                fn rr_get_config(&self) -> Rr_Desc_Enum {
                    Rr_Desc_Enum {
                        arms: &[ #(#config),* ]
                    }

                }
            }
        };

        let enum_arm = {
            let len = x.variants.len();
            quote::quote! {
                fn rr_enum_arm(&mut self) -> Result<usize, Rr_Err> {
                    Ok(#len)
                }
            }
        };

        let enum_switch = {
            let mut arms = vec![];
            for (i, (ident, fields)) in x.variants.iter().enumerate() {
                match fields {
                    Rast_Fields::Unnamed(x) => {
                        let mut parts = vec![];
                        for _ in 0..x.len() {
                            parts.push(quote::quote! { Default::default() });
                        }
                        arms.push(quote::quote! { #i => { * self = #name_enum :: #ident ( #(#parts),* ) } });
                    }
                    Rast_Fields::Named(x) => {
                        let mut parts = vec![];
                        for (ident, _v) in x.iter() {
                            parts.push(quote::quote! { #ident: Default::default() })
                        }
                        arms.push(quote::quote! { #i => { *self = #name_enum :: #ident { #(#parts),* } } });
                    }
                    Rast_Fields::Unit => {}
                }
            }
            quote::quote! {
                fn rr_enum_switch(&mut self, idx: usize) -> Result<(), Rr_Err> {
                    match idx {
                        #(#arms)*
                        _ => { return Err(Rr_Err::Enum_Switch{ val: idx }) ; }
                    }
                    Ok(())
                }
            }
        };

        let get_inner = |s| {
            let mut arms = vec![];
            for (arm_idx, (ident, fields)) in x.variants.iter().enumerate() {
                match fields {
                    Rast_Fields::Unnamed(x) => {
                        let mut lhs = vec![];
                        let mut rhs = vec![];
                        for i in 0..x.len() {
                            let nm = proc_macro2::Ident::new(&format!("v_{:?}", i), name_enum.span());
                            lhs.push(nm.clone());
                            rhs.push(quote::quote! { #i => { return Ok((#nm).#s ()); } });
                        }
                        arms.push(quote::quote! {
                            #name_enum::#ident ( #(#lhs),* ) => {
                                match n {
                                    #(#rhs)*
                                    _ => return Err(Rr_Err::Enum_Get_Oob { arm_idx: #arm_idx, val: n })
                                }
                            }
                        });
                    }
                    Rast_Fields::Named(x) => {
                        let mut lhs = vec![];
                        let mut rhs = vec![];
                        for (i, (ident, _ty)) in x.iter().enumerate() {
                            let nm = proc_macro2::Ident::new(&format!("v_{:?}", i), name_enum.span());
                            lhs.push(quote::quote! { # ident : #nm });
                            rhs.push(quote::quote! { #i => { return Ok((#nm).#s ()); } });
                        }
                        arms.push(quote::quote! {
                            #name_enum::#ident { #(#lhs),* } => {
                                match n {
                                    #(#rhs)*
                                    _ => return Err(Rr_Err::Enum_Get_Oob { arm_idx: #arm_idx, val: n })
                                }
                            }
                        });
                    }
                    Rast_Fields::Unit => arms.push(quote::quote! {
                        #name_enum::#ident  => { return Err(Rr_Err::Enum_Get_Unit ) ; }
                    }),
                }
            }
            quote::quote! {
                    match self { #(#arms)* }
            }
        };

        let rr_get = get_inner(syn::Ident::new("rr_to_any_ref", x.name.span()));
        let rr_get_mut = get_inner(syn::Ident::new("rr_to_mut_any_ref", x.name.span()));

        let (gen_impl, gen_ty, _c) = &x.generics.split_for_impl();
        let w_clauses = {
            let mut out = vec![];
            for x in x.generics.params.iter() {
                match x {
                    GenericParam::Lifetime(_) => {}
                    GenericParam::Type(t) => {
                        let nm = &t.ident;
                        out.push(quote::quote! { #nm : Rr_Any_T });
                    }
                    GenericParam::Const(_) => {}
                }
            }
            out
        };

        quote::quote! {


            impl #gen_impl #name_enum #gen_ty where #(#w_clauses),*  {
                pub fn rr_foo() -> u32 {
                    42
                }

            }


            impl #gen_impl Rr_Any_T for #name_enum #gen_ty where #(#w_clauses),*  {

                fn rr_to_any_ref<'a>(&'a self) -> Rr_Any_Ref<'a> {
                    Rr_Any_Ref::Enum(self as _)
                }

                fn rr_to_mut_any_ref<'a>(&'a mut self) -> Rr_Mut_Any_Ref<'a> {
                    Rr_Mut_Any_Ref::Enum(self as _)
                }
            }

            impl #gen_impl Rr_Enum_T for #name_enum #gen_ty where #(#w_clauses),* {
                #get_config
                #enum_arm
                #enum_switch

                fn rr_get<'a>(&'a self, n: usize) -> Result<Rr_Any_Ref<'a>, Rr_Err> {
                    #rr_get
                }

                fn rr_get_mut<'a>(&'a mut self, n: usize) -> Result<Rr_Mut_Any_Ref<'a>, Rr_Err> {
                    #rr_get_mut
                }
            }

        }
    }
}
