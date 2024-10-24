use serde_textual::assert_textual;

fn main() {
    assert_textual!(SimpleEnum::VariantA, "VariantA");
    assert_textual!(SimpleEnum::VariantB, "variant-b");
}

// Those are not imported so the generated code cannot call those trait methods.
#[derive(
    serde::Serialize,
    serde::Deserialize,
    serde_textual::DisplaySerde,
    serde_textual::FromStrSerde,
    Debug,
    PartialEq,
)]
pub enum SimpleEnum {
    VariantA,
    #[serde(rename = "variant-b")]
    VariantB,
}

#[test]
fn test_enum() {
    main();
}
