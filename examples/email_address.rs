use serde::{Deserialize, Serialize};
use serde_textual::{assert_textual, DisplaySerde, FromStrSerde};

fn main() {
    let email = NewEmailAddress("user@example.com".parse().unwrap());
    let s = "user@example.com";

    assert_textual!(email, s);
}

#[derive(FromStrSerde, DisplaySerde, Serialize, Deserialize, Debug, PartialEq)]
#[serde(transparent)]
struct NewEmailAddress(email_address::EmailAddress);

#[test]
fn test_email_address() {
    main();
}
