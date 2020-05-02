extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let builder_struct = gen_struct(&name, &input.data);

    let q = quote! {
        impl #name {
           pub fn builder() {}
        }
    };
    eprintln!("{:#?}", builder_struct);
    proc_macro::TokenStream::from(q)
}

fn gen_struct(name: &Ident, data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let new_fields = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    let field_type = &f.ty;
                    quote_spanned! {f.span() =>
                        #name: Option<#field_type>
                    }
                });
                quote! {
                    pub struct #name Builder {
                        #(#new_fields)*,
                    }
                }
            }
            _ => unimplemented!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };
    TokenStream::new()
}
