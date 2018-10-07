use extend::ToFromC;
use ffi;
use na::{Point3, Vector3};

/// A ball simulation.
///
/// # Example
///
/// ```
/// # extern crate chip;
/// # extern crate nalgebra as na;
/// # use na::{Point3, Vector3};
/// # use chip::Ball;
/// let mut ball = Ball::new();
/// ball.set_pos(Point3::new(0.0, 0.0, 100.0));
/// ball.set_vel(Vector3::new(300.0, 400.0, 500.0));
///
/// ball.step(1.0 / 120.0);
/// println!("{:?}", ball.pos());
/// ```
pub struct Ball(ffi::Ball);

impl Ball {
    /// Creates a `Ball`.
    pub fn new() -> Self {
        Ball(unsafe { ffi::Ball::new() })
    }

    /// Gets the ball's position.
    pub fn pos(&self) -> Point3<f32> {
        Point3::from_c(self.0.x)
    }

    /// Sets the ball's position.
    pub fn set_pos(&mut self, pos: Point3<f32>) {
        self.0.x = pos.to_c();
    }

    /// Gets the ball's velocity.
    pub fn vel(&self) -> Vector3<f32> {
        Vector3::from_c(self.0.v)
    }

    /// Sets the ball's velocity.
    pub fn set_vel(&mut self, vel: Vector3<f32>) {
        self.0.v = vel.to_c();
    }

    /// Gets the ball's angular velocity.
    pub fn omega(&self) -> Vector3<f32> {
        Vector3::from_c(self.0.w)
    }

    /// Sets the ball's angular velocity.
    pub fn set_omega(&mut self, omega: Vector3<f32>) {
        self.0.w = omega.to_c();
    }

    /// Gets the time elapsed in the simulation.
    pub fn t(&self) -> f32 {
        self.0.t
    }

    /// Sets the time elapsed in the simulation.
    pub fn set_t(&mut self, t: f32) {
        self.0.t = t;
    }

    /// Simulates the next `dt` seconds, and updates the ball's physics values.
    pub fn step(&mut self, dt: f32) {
        unsafe { self.0.step(dt) };
    }
}

#[cfg(test)]
mod tests {
    use ball::Ball;
    use na::{Point3, Vector3};

    #[test]
    fn ball() {
        let mut ball = Ball::new();
        ball.set_pos(Point3::new(0.0, 0.0, 100.0));
        ball.set_vel(Vector3::new(1000.0, 0.0, 0.0));
        ball.step(1.0 / 120.0);
        println!("{:?}", ball.pos());
        assert!(1.0 < ball.pos().x && ball.pos().x < 10.0);
        assert!(99.0 < ball.pos().z && ball.pos().z < 99.99);
    }
}
