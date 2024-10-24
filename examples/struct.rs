use serde_textual::assert_textual;

fn main() {
    let v = SimpleStruct {
        field: "value".to_string(),
    };
    assert_textual!(v, "value");
}

#[derive(
    serde::Serialize,
    serde::Deserialize,
    serde_textual::DisplaySerde,
    serde_textual::FromStrSerde,
    Debug,
    PartialEq,
)]
#[serde(transparent)]
struct SimpleStruct {
    field: String,
}

#[test]
fn test_struct() {
    main();
}
