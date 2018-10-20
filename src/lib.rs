/*!
This is a Rust binding to [Lobot], created by Sam Mish (aka chip). Lobot is a
Rocket League bot, and a collection of tools used to make predictions about how
the car and ball behave. Implementation notes can be found on chip's [blog].

[Lobot]: https://github.com/samuelpmish/Utilities
[blog]: https://samuelpmish.github.io/notes/RocketLeague/
*/

#![cfg_attr(feature = "strict", deny(warnings, missing_docs))]

extern crate nalgebra as na;

pub use ball::Ball;
pub use car::{max_curvature, max_speed, Car};
pub use ffi::Input;

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
