/*!
This is a Rust binding to [RLUtilities], created by Sam Mish (aka chip).
RLUtilities aims to provide a way for members of the RLBot community to
contribute their ideas to a common set of tools for arithmetic, simulation, and
car controllers. Implementation notes can be found on chip's [blog].

[RLUtilities]: https://github.com/samuelpmish/RLUtilities
[blog]: https://samuelpmish.github.io/notes/RocketLeague/
*/

#![cfg_attr(feature = "strict", deny(warnings, missing_docs))]

extern crate nalgebra as na;

pub use ball::Ball;
pub use car::{max_curvature, max_speed, Car};
pub use ffi::Input;
pub use mat::euler_rotation;

mod ball;
mod car;
mod extend;
#[allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    missing_docs
)]
mod ffi;
mod ffi_types;
mod mat;
