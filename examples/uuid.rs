use serde::{Deserialize, Serialize};
use serde_textual::{assert_textual, DisplaySerde, FromStrSerde};

fn main() {
    let new_uuid = NewUuid("95c639c4-0c2f-4026-900c-6144c6c39b1f".parse().unwrap());
    let s = "95c639c4-0c2f-4026-900c-6144c6c39b1f";

    assert_textual!(new_uuid, s);
}

#[derive(FromStrSerde, DisplaySerde, Serialize, Deserialize, Debug, PartialEq)]
#[serde(transparent)]
struct NewUuid(uuid::Uuid);

#[test]
fn test_uuid() {
    main();
}
