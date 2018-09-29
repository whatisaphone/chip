# chip

This is a Rust binding to [Lobot], created by Sam Mish (aka chip).
Implementation notes can be found on his [blog].

[Lobot]: https://github.com/samuelpmish/Utilities
[blog]: https://samuelpmish.github.io/notes/RocketLeague/

## Building

Building this is a somewhat messy process, sorry.

* Prerequisites
  * Rust
  * bindgen (which requires clang)
  * [Lobot]'s prerequisites (cmake and Visual Studio)

* `git clone` Lobot to a convenient place.

* Generate bindings

    ```
    chip=<path-to-lobot>
    bindgen \
        upstream/chip.hpp \
        -o src/ffi.rs \
        --generate-inline-functions \
        --no-layout-tests \
        --with-derive-default \
        --raw-line '#![allow(dead_code, non_camel_case_types, non_snake_case, missing_docs)]' \
        --raw-line '' \
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

* Build [Lobot], and you'll end up with `bot_utils.lib`. Copy `bot_utils.lib`
  into `upstream/`.

* `cargo test`, and pray to your deity of choice.
