# serde-textual

[![GitHub](https://img.shields.io/badge/GitHub-ryo33/serde--textual-222222)](https://github.com/ryo33/serde-textual)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/serde-textual)](https://crates.io/crates/serde-textual)
[![docs.rs](https://img.shields.io/docsrs/serde-textual)](https://docs.rs/serde-textual)

derive `Display`/`FromStr` by using `serde`'s `Serialize`/`Deserialize`

## Usage examples

### Enum using `serde`'s `rename_all` or `rename`

```rust
use serde::{Serialize, Deserialize};
use serde_textual::{FromStrSerde, DisplaySerde};

#[derive(FromStrSerde, DisplaySerde, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Kind {
    A,
    B,
}

assert_eq!("a".parse::<Kind>().unwrap(), Kind::A);
assert_eq!(Kind::B.to_string(), "b");
```

### New types with `Uuid`, `Url`, `EmailAddress`, etc

Don't forget to activate `serde` feature of those crates.

```rust
use serde::{Serialize, Deserialize};
use serde_textual::{FromStrSerde, DisplaySerde};

#[derive(FromStrSerde, DisplaySerde, Serialize, Deserialize, Debug, PartialEq)]
#[serde(transparent)]
struct NewUuid(uuid::Uuid);

let new_uuid = NewUuid("95c639c4-0c2f-4026-900c-6144c6c39b1f".parse().unwrap());
let s = "95c639c4-0c2f-4026-900c-6144c6c39b1f";

assert_eq!(new_uuid, s.parse::<NewUuid>().unwrap());
assert_eq!(s, new_uuid.to_string());
```

In the above example, `NewUuid`'s `Display`/`FromStr` delegates to `NewUuid`'s `Serialize`/`Deserialize` that delegates to `Uuid`'s `Serialize`/`Deserialize`. If you need to implement `NewUuid`'s `Display`/`FromStr` to directly delegate to `Uuid`'s `Display`/`FromStr` (for compile-time or runtime performance reason), use other crates that supports `Display`/`FromStr` delegation, or do it manually.

### [Advanced] Comma separated values of enum variants

`serde_with`'s [`StringWithSeparator<Separator, T>`](https://docs.rs/serde_with/3.11.0/serde_with/struct.StringWithSeparator.html) requires `T` to implement `Display`/`FromStr`. It's convenient to use `serde-textual` to implement `Display`/`FromStr` for the `T`.

```rust
use serde::{Deserialize, Serialize};
use serde_textual::{assert_textual, DisplaySerde, FromStrSerde};
use serde_with::{formats::CommaSeparator, serde_as, StringWithSeparator};

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

let required_fields = RequiredFields(vec![Field::Id, Field::Name, Field::CreatedAt]);
let s = "id,name,createdAt";

assert_eq!(required_fields, s.parse::<RequiredFields>().unwrap());
assert_eq!(s, required_fields.to_string());
```

## License

Licensed under either of

- Apache-2.0
- MIT

at your option.
