use extend::ToFromC;
use ffi;
use na::Vector3;

pub struct Ball(ffi::Ball);

impl Ball {
    pub fn new() -> Self {
        Ball(unsafe { ffi::Ball::new() })
    }

    pub fn pos(&self) -> Vector3<f32> {
        Vector3::from_c(self.0.x)
    }

    pub fn set_pos(&mut self, pos: Vector3<f32>) {
        self.0.x = pos.to_c();
    }

    pub fn vel(&self) -> Vector3<f32> {
        Vector3::from_c(self.0.v)
    }

    pub fn set_vel(&mut self, vel: Vector3<f32>) {
        self.0.v = vel.to_c();
    }

    pub fn omega(&self) -> Vector3<f32> {
        Vector3::from_c(self.0.w)
    }

    pub fn set_omega(&mut self, omega: Vector3<f32>) {
        self.0.w = omega.to_c();
    }

    pub fn t(&self) -> f32 {
        self.0.t
    }

    pub fn set_t(&mut self, t: f32) {
        self.0.t = t;
    }

    pub fn step(&mut self, dt: f32) {
        unsafe { self.0.step(dt) };
    }
}

#[cfg(test)]
mod tests {
    use ball::Ball;
    use na::Vector3;

    #[test]
    fn ball() {
        let mut ball = Ball::new();
        ball.set_pos(Vector3::new(0.0, 0.0, 100.0));
        ball.set_vel(Vector3::new(1000.0, 0.0, 0.0));
        ball.step(1.0 / 120.0);
        println!("{:?}", ball.pos());
        assert!(1.0 < ball.pos().x && ball.pos().x < 10.0);
        assert!(99.0 < ball.pos().z && ball.pos().z < 99.99);
    }
}
