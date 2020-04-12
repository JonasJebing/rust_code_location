# code_location

A library to automatically acquire a code location in a rust source code file.

## Install

```toml
[dependencies]
code_location = "1.0"
```

## Usage

```rust
use code_location::{code_location, CodeLocation};

fn main() {
    println!("I am printing from {}", code_location!());
}
```

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