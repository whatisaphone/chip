# chip

[![crates.io](https://img.shields.io/crates/v/chip.svg)](https://crates.io/crates/chip)
[![docs](https://docs.rs/chip/badge.svg)](https://docs.rs/chip/)
[![Build Status](https://travis-ci.org/whatisaphone/chip.svg?branch=master)](https://travis-ci.org/whatisaphone/chip)

This is a Rust binding to [Lobot], created by Sam Mish (aka chip). Lobot is a
Rocket League bot, and a collection of tools used to make predictions about how
the car and ball behave. Implementation notes can be found on chip's [blog].

[Lobot]: https://github.com/samuelpmish/Utilities
[blog]: https://samuelpmish.github.io/notes/RocketLeague/

## Development

### Prerequisites

* Windows
* Rust
* bindgen (which requires clang)
* [Lobot]
  * It has its own prerequisites (cmake, Visual Studio, Python 3)
  * Build `bot_utils.lib` and copy it into `upstream/`.

### To generate bindings

```sh
chip=<path-to-lobot>
bindgen \
    upstream/chip.hpp \
    -o src/ffi.rs \
    --generate-inline-functions \
    --no-layout-tests \
    --with-derive-default \
    --raw-line 'use ffi_types::{mat3, vec3};' \
    --whitelist-type Ball \
    --whitelist-type Car \
    --blacklist-type mat3 \
    --blacklist-type vec3 \
    --blacklist-type '^mat_mat<.+' \
    --blacklist-type '^vec_vec<.+' \
    -- \
    -I "$chip/cpp/inc"
```
