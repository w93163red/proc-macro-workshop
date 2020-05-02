extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let _builder = gen_builder(&input.data);

    let q = quote! {
        impl #name {
           pub fn builder() {}
        }
    };
    eprintln!("{:#?}", q);
    proc_macro::TokenStream::from(q)
}

fn gen_builder(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {}
            Fields::Unnamed(ref fields) => {}

            Fields::Unit => {}
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };
    TokenStream::new()
}
