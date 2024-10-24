use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(input: syn::DeriveInput) -> TokenStream {
    let ident = input.ident;

    quote! {
        impl ::core::fmt::Display for #ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::serde::ser::Serialize::serialize(self, f)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::parse_quote;

    use super::*;

    #[test]
    fn simple_enum() {
        let input = parse_quote! {
            enum SimpleEnum {
                Variant1,
                Variant2,
            }
        };
        let expected = quote! {
            impl ::core::fmt::Display for SimpleEnum {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::serde::ser::Serialize::serialize(self, f)
                }
            }
        };
        assert_eq!(generate(input).to_string(), expected.to_string());
    }
}
