use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(input: syn::DeriveInput) -> TokenStream {
    let ident = input.ident;
    quote! {
        impl ::core::str::FromStr for #ident {
            type Err = ::serde::de::value::Error;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                ::serde::Deserialize::deserialize(
                    ::serde::de::value::StrDeserializer::new(s),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn simple_enum() {
        let input = parse_quote! {
            enum SimpleEnum {
                Variant1,
                Variant2,
            }
        };
        let expected = quote! {
            impl ::core::str::FromStr for SimpleEnum {
                type Err = ::serde::de::value::Error;

                #[inline]
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    ::serde::Deserialize::deserialize(
                        ::serde::de::value::StrDeserializer::new(s),
                    )
                }
            }
        };
        assert_eq!(generate(input).to_string(), expected.to_string());
    }
}
