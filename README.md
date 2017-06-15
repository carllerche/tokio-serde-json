# Tokio / Serde bindings for JSON

Utilities needed to easily implement a Tokio JSON transport using [serde] for
JSON serialization and deserialization of frame values.

[Documentation](https://carllerche.github.io/tokio-serde-json/tokio_serde_json/index.html)

## Usage

To use `tokio-serde-json`, first add this to your `Cargo.toml`:

```toml
[dependencies]
tokio-serde-json = { git = "https://github.com/carllerche/tokio-serde-json" }
```

Next, add this to your crate:

```rust
extern crate tokio_serde_json;

use tokio_serde_json::{ReadJson, WriteJson};
```

[serde]: https://serde.rs

# License

`tokio-serde-json` is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0), with portions covered by various
BSD-like licenses.

See LICENSE-APACHE, and LICENSE-MIT for details.
