# code_location

A library to automatically acquire a code location in a rust source code file.

## UNMAINTAINED

This library is no longer maintained.
Although it does work fine, I recommend using `std::panic::Location` instead.

## Install

```toml
[dependencies]
code_location = "1.1"
```

## Usage

```rust
use code_location::{code_location, CodeLocation};

fn main() {
    // `I am printing from src/main.rs:5:39`
    println!("I am printing from {}", code_location!());

    let code_location: CodeLocation = code_location!();

    // `The location above is: src/main.rs:7:39`
    println!("The location above is: {}", code_location);
}
```

## `serde` serialization and deserialization support

[`serde`] support can be enabled with the `"serde"` feature:

```toml
[dependencies]
code_location = { version = "1.1", features = ["serde"] }
```

[`serde`]: https://crates.io/crates/serde

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in code_location by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
