extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let builder = gen_builder(&input.data);
    println!("{:?}", name);
    TokenStream::new()
}

fn gen_builder(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    eprintln!("{:?}", name);
                });
            }
            _ => {}
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    };
    TokenStream::new()
}
