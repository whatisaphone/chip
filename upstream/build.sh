#!/usr/bin/env bash

set -euo pipefail

here="$(dirname "$0")"
root="$here"/..

# Adapted from `RLUtilities/build.sh`:
(
    cd "$here"/RLUtilities/RLUtilities/cpp
    mkdir -p build
    cd build
    cmake .. -G"Visual Studio 15 2017 Win64"
    cmake --build . --config Release
)
cp "$here"/RLUtilities/RLUtilities/cpp/build/Release/bot_utils.lib "$here"

args=(
    "$here"/chip.hpp
    -o "$root"/src/ffi.rs
    --generate-inline-functions
    --no-layout-tests
    --with-derive-default
    # ball.h
    --whitelist-type Ball
    # bvh.h
    --opaque-type bvh
    # Car.h
    --whitelist-type Car
    --whitelist-function max_curvature
    --whitelist-function max_speed
    # mat.h
    --blacklist-function '^mat_mat<.*'
    # vec.h
    --blacklist-function '^vec_vec<.*'
    # Misc
    --opaque-type '^std::.*'
    --blacklist-type '^std::.*'
    --
    -I "$here"/RLUtilities/RLUtilities/cpp/inc
)
bindgen "${args[@]}"

# Bindgen interprets vec/mat incorrectly and thinks the floats are ints. Fix it
# with a very blunt hammer.
sed -i 's/\[\s*u32\s*;/\[f32;/g' "$root"/src/ffi.rs
