use serde_textual::assert_textual;

fn main() {
    assert_textual!(SimpleEnum::VariantA, "VariantA");
    assert_textual!(SimpleEnum::VariantB, "variant-b");
}

#[derive(
    serde::Serialize,
    serde::Deserialize,
    serde_textual::Display,
    serde_textual::FromStr,
    Debug,
    PartialEq,
)]
pub enum SimpleEnum {
    VariantA,
    #[serde(rename = "variant-b")]
    VariantB,
}
