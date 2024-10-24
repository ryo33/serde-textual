use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};

pub struct AssertTextualInput {
    value: syn::Expr,
    #[allow(dead_code)]
    comma: syn::Token![,],
    textual: syn::Expr,
}

impl Parse for AssertTextualInput {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        Ok(Self {
            value: input.parse()?,
            comma: input.parse()?,
            textual: input.parse()?,
        })
    }
}

pub fn generate(input: AssertTextualInput) -> TokenStream {
    let AssertTextualInput { value, textual, .. } = input;
    quote! {
        {
            assert_eq!(#value, #textual.parse().unwrap());
            assert_eq!(#value.to_string(), #textual);
        }
    }
}
