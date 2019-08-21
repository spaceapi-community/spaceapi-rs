# Rust SpaceAPI Implementation

[![CircleCI](https://circleci.com/gh/spaceapi-community/spaceapi-server-rs/tree/master.svg?style=shield)](https://circleci.com/gh/spaceapi-community/spaceapi-server-rs/tree/master)
[![Crates.io Version](https://img.shields.io/crates/v/spaceapi.svg)](https://crates.io/crates/spaceapi)
[![Crates.io Downloads](https://img.shields.io/crates/d/spaceapi.svg)](https://crates.io/crates/spaceapi)

This is an implementation of the [SpaceAPI](https://spacedirectory.org/) v0.13
in Rust. It contains both the type definitions as well as tools for
serialization and deserialization to/from JSON using Serde.

- Crate Documentation: https://docs.rs/spaceapi/
- SpaceAPI Documentation: https://spaceapi.io/pages/docs.html

This library requires Rust 1.31.0 or newer.


## Usage

Add `spaceapi` to your `Cargo.toml`:

    [dependencies]
    spaceapi = "^0.5"


## Docs

You can build docs with `make docs`. Find them in the `target/doc/` directory.


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
