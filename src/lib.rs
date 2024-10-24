use syn::parse_macro_input;

mod assert_textual;
mod display;
mod from_str;

#[proc_macro_derive(Display)]
pub fn derive_display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    display::generate(parse_macro_input!(input as syn::DeriveInput)).into()
}

#[proc_macro_derive(FromStr)]
pub fn derive_from_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from_str::generate(parse_macro_input!(input as syn::DeriveInput)).into()
}

#[proc_macro]
pub fn assert_textual(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    assert_textual::generate(parse_macro_input!(
        input as assert_textual::AssertTextualInput
    ))
    .into()
}
