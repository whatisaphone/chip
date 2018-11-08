# chip

[![crates.io](https://img.shields.io/crates/v/chip.svg)](https://crates.io/crates/chip)
[![docs](https://docs.rs/chip/badge.svg)](https://docs.rs/chip/)
[![Build Status](https://travis-ci.org/whatisaphone/chip.svg?branch=master)](https://travis-ci.org/whatisaphone/chip)

This is a Rust binding to [RLUtilities], created by Sam Mish (aka chip).
RLUtilities aims to provide a way for members of the RLBot community to
contribute their ideas to a common set of tools for arithmetic, simulation, and
car controllers. Implementation notes can be found on chip's [blog].

[RLUtilities]: https://github.com/samuelpmish/RLUtilities
[blog]: https://samuelpmish.github.io/notes/RocketLeague/

## Development

### Prerequisites

* Windows
* Rust
* [bindgen]
* bindgen's prerequisites (clang)
* [RLUtilities]'s prerequisites (cmake, Visual Studio, Python 3)

[bindgen]: https://rust-lang-nursery.github.io/rust-bindgen/

### Getting started

You'll need to get the upstream C++ compiling successfully. Hopefully it's as
simple as running `upstream/build.sh` and then `cargo test`. Bindgen will output
a generous serving of errors, but as long as the tests pass, the errors from
bindgen can be ignored.
