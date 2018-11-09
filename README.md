# Tokio / Serde bindings for JSON

Utilities needed to easily implement a Tokio JSON transport using [serde] for
JSON serialization and deserialization of frame values.

[Documentation](https://carllerche.github.io/tokio-serde-json/tokio_serde_json/index.html)

## Usage

To use `tokio-serde-json`, first add this to your `Cargo.toml`:

```toml
[dependencies]
tokio-serde-json = "0.2"
```

Next, add this to your crate:

```rust
extern crate tokio_serde_json;

use tokio_serde_json::{ReadJson, WriteJson};
```

[serde]: https://serde.rs

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `tower-web` by you, shall be licensed as MIT, without any
additional terms or conditions.
