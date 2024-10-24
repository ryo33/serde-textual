use syn::parse_macro_input;

mod assert_textual;
mod display;
mod from_str;

#[proc_macro_derive(DisplaySerde)]
/// Derive `Display` trait by using `serde::Serialize` implementation.
pub fn derive_display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    display::generate(parse_macro_input!(input as syn::DeriveInput)).into()
}

#[proc_macro_derive(FromStrSerde)]
/// Derive `FromStr` trait by using `serde::Deserialize` implementation.
pub fn derive_from_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from_str::generate(parse_macro_input!(input as syn::DeriveInput)).into()
}

#[proc_macro]
/// Assert `Display`/`FromStr` implementation is consistent by converting between value and str and asserting the result is the same.
///
/// ```
/// use serde_textual::assert_textual;
///
/// let value = true;
/// let textual = "true";
/// assert_textual!(value, textual);
///
/// // equivalent to the following two statements:
/// assert_eq!(value, textual.parse().unwrap());
/// assert_eq!(value.to_string(), textual);
/// ```
pub fn assert_textual(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    assert_textual::generate(parse_macro_input!(
        input as assert_textual::AssertTextualInput
    ))
    .into()
}
