use super::*;

pub struct Enum_USize {}

impl Enum_USize {
    pub fn new() -> Enum_USize {
        Enum_USize {}
    }

    pub fn output(self, outer_Ident: &proc_macro2::Ident, vs: &Vec<Ident>) -> proc_macro2::TokenStream {
        let num_Id = (0..vs.len()).map(|x| quote! { #x }).collect::<Vec<_>>();
        let first = vs.first().unwrap().clone();
        let len = vs.len();
        quote! {
            impl Enum_Usize_T_ for #outer_Ident {
                const _enum_len: usize = #len;
                fn _to_usize(&self) -> usize {
                    match self {
                        #( #outer_Ident::#vs  =>  #num_Id, )*
                    }
                }

                fn _from_usize(t: usize) -> Self {
                    match t {
                        #( #num_Id => #outer_Ident::#vs, )*
                        _ => #outer_Ident::#first ,
                    }
                }
            }
        }
    }

    pub fn proc_enum_usize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
        let DeriveInput { ident: outer_Ident, data, .. } = parse_macro_input!(input);

        let output = match data {
            syn::Data::Enum(DataEnum { variants, .. }) => {
                let builder = Enum_USize::new();
                let mut vs = vec![];
                for x in variants.iter() {
                    match x.fields {
                        Fields::Unit => {}
                        Fields::Named(_) => {
                            todo!("named field")
                        }
                        Fields::Unnamed(_) => {
                            todo!("unnamed field")
                        }
                    }
                    vs.push(x.ident.clone());
                }
                let t = builder.output(&outer_Ident, &vs);
                t
            }
            _ => {
                todo!("error in x_enum_Id")
            }
        };
        output.into()
    }
}
