//! There are a few types that `bindgen` translates incorrectly, so we blacklist
//! them and instead define them by hand here.

#![allow(non_camel_case_types)]

pub type vec3 = [f32; 3];
pub type mat3 = [f32; 9];
