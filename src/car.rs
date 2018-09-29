use extend::ToFromC;
use ffi;
use na::{Matrix3, Rotation3, Vector3};

/// A car simulation.
///
/// # Example
///
/// ```
/// # extern crate chip;
/// # extern crate nalgebra as na;
/// # use na::{Rotation3, Vector3};
/// # use chip::{Car, Input};
/// let mut car = Car::new();
/// car.set_pos(Vector3::new(0.0, 0.0, 17.01));
/// car.set_vel(Vector3::new(300.0, 400.0, 500.0));
/// car.set_theta(Rotation3::identity());
/// car.set_on_ground(true);
///
/// let input = Input {
///     steer: 1.0,
///     throttle: 1.0,
///     ..Input::default()
/// };
/// car.step(input, 1.0 / 120.0);
/// println!("{:?}", car.pos());
/// ```
pub struct Car(ffi::Car);

impl Car {
    /// Creates a `Car`.
    pub fn new() -> Self {
        Car(unsafe { ffi::Car::new() })
    }

    /// Simulates the next `dt` seconds, and updates the car's physics values.
    pub fn step(&mut self, input: ffi::Input, dt: f32) {
        unsafe { self.0.step(input, dt) };
    }

    /// Gets the car's position.
    pub fn pos(&self) -> Vector3<f32> {
        Vector3::from_c(self.0.x)
    }

    /// Sets the car's position.
    pub fn set_pos(&mut self, pos: Vector3<f32>) {
        self.0.x = pos.to_c();
    }

    /// Gets the car's velocity.
    pub fn vel(&self) -> Vector3<f32> {
        Vector3::from_c(self.0.v)
    }

    /// Sets the car's velocity.
    pub fn set_vel(&mut self, vel: Vector3<f32>) {
        self.0.v = vel.to_c();
    }

    /// Gets the car's angular velocity.
    pub fn omega(&self) -> Vector3<f32> {
        Vector3::from_c(self.0.w)
    }

    /// Sets the car's angular velocity.
    pub fn set_omega(&mut self, omega: Vector3<f32>) {
        self.0.w = omega.to_c();
    }

    /// Gets the car's rotation.
    pub fn theta(&self) -> Rotation3<f32> {
        Rotation3::from_matrix_unchecked(Matrix3::from_c(self.0.o))
    }

    /// Sets the car's rotation.
    pub fn set_theta(&mut self, theta: Rotation3<f32>) {
        self.0.o = theta.matrix().to_c();
    }

    /// Gets whether the car's wheels are on the ground.
    pub fn on_ground(&self) -> bool {
        self.0.on_ground
    }

    /// Sets whether the car's wheels are on the ground.
    pub fn set_on_ground(&mut self, on_ground: bool) {
        self.0.on_ground = on_ground;
    }
}

#[cfg(test)]
mod tests {
    use car::Car;
    use na::Vector3;

    #[test]
    fn car() {
        let mut car = Car::new();
        car.set_on_ground(true);
        car.set_pos(Vector3::zeros());
        car.set_vel(Vector3::new(1000.0, 0.0, 0.0));
        car.step(Default::default(), 1.0 / 120.0);
        println!("{:?}", car.pos());
        assert!(1.0 < car.pos().x && car.pos().x < 10.0);
    }
}
