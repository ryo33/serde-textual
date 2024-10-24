use serde::{Deserialize, Serialize};
use serde_textual::{assert_textual, DisplaySerde, FromStrSerde};

fn main() {
    let url = WebUrl("https://www.example.com/".parse().unwrap());
    let s = "https://www.example.com/";

    assert_textual!(url, s);
}

#[derive(FromStrSerde, DisplaySerde, Serialize, Deserialize, Debug, PartialEq)]
#[serde(transparent)]
struct WebUrl(url::Url);

#[test]
fn test_url() {
    main();
}
