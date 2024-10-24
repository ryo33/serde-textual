use serde::{Deserialize, Serialize};
use serde_textual::{assert_textual, DisplaySerde, FromStrSerde};
use serde_with::{formats::CommaSeparator, serde_as, StringWithSeparator};

fn main() {
    let required_fields = RequiredFields(vec![Field::Id, Field::Name, Field::CreatedAt]);
    let s = "id,name,createdAt";

    assert_textual!(required_fields, s);
}

#[serde_with::serde_as]
#[derive(FromStrSerde, DisplaySerde, Serialize, Deserialize, Debug, PartialEq)]
#[serde(transparent)]
struct RequiredFields(#[serde_as(as = "StringWithSeparator<CommaSeparator, Field>")] Vec<Field>);

#[derive(FromStrSerde, DisplaySerde, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
enum Field {
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}

#[test]
fn test_string_with_separator() {
    main();
}
