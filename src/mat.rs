use extend::ToFromC;
use ffi;
use na::{Matrix3, Rotation3, Vector3};

/// Converts the given (pitch, yaw, roll) to a rotation matrix. The rotations
/// are applied in ZYX order, which matches Rocket League.
///
/// # Example
///
/// ```
/// # extern crate chip;
/// # extern crate nalgebra as na;
/// # use chip::euler_rotation;
/// # use na::Vector3;
/// # use std::f32::consts::PI;
/// let pitch = 0.0;
/// let yaw = PI / 2.0;
/// let roll = 0.0;
/// let pyr = Vector3::new(pitch, yaw, roll);
/// let rotation_matrix = euler_rotation(&pyr);
/// ```
pub fn euler_rotation(pyr: &Vector3<f32>) -> Rotation3<f32> {
    Rotation3::from_matrix_unchecked(Matrix3::from_c(unsafe { ffi::euler_rotation(&pyr.to_c()) }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f32 = 0.001;

    #[test]
    fn test_euler_rotation() {
        let expected = Rotation3::identity();
        let actual = euler_rotation(&Vector3::new(0.0, 0.0, 0.0));
        assert!(expected.rotation_to(&actual).angle() < EPS);
    }
}
