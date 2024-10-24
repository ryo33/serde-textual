# serde-textual

[![GitHub](https://img.shields.io/badge/GitHub-ryo33/serde--textual-222222)](https://github.com/ryo33/serde-textual)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/serde-textual)](https://crates.io/crates/serde-textual)
[![docs.rs](https://img.shields.io/docsrs/serde-textual)](https://docs.rs/serde-textual)

derive `FromStr` and `Display` by using `serde` impl

## Usage

```rust
use serde_textual::{FromStr, Display};

#[derive(FromStr, Display, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Kind {
    A,
    B,
}
```

## Show case

### Enum using `serde`'s `rename_all` and `rename`

### New types with UUID, Url, EmailAddress, etc

### [Advanced] Comma separated values of enum variants

## License

Licensed under either of

- Apache-2.0
- MIT

at your option.
