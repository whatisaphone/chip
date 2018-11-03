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
* bindgen (which requires clang)
* [RLUtilities]
  * It has its own prerequisites (cmake, Visual Studio, Python 3)
  * Build `bot_utils.lib` and copy it into `upstream/`.

### To generate bindings

Unfortunately the blacklist of `vec_vec` does not work, so a few invalid
declarations need to be removed by hand from the generated file :(

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
    --whitelist-function max_curvature \
    --whitelist-function max_speed \
    --blacklist-type mat3 \
    --blacklist-type vec3 \
    --blacklist-type '^mat_mat<.+' \
    --blacklist-type '^vec_vec<.+' \
    -- \
    -I "$chip/RLUtilities/cpp/inc"
```
