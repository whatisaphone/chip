use extend::ToFromC;
use ffi;
use na::{Matrix3, Rotation3, Vector3};

pub struct Car(ffi::Car);

impl Car {
    pub fn new() -> Self {
        Car(unsafe { ffi::Car::new() })
    }

    pub fn step(&mut self, in_: ffi::Input, dt: f32) {
        unsafe { self.0.step(in_, dt) };
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

    pub fn theta(&self) -> Rotation3<f32> {
        Rotation3::from_matrix_unchecked(Matrix3::from_c(self.0.o))
    }

    pub fn set_theta(&mut self, theta: Rotation3<f32>) {
        self.0.o = theta.matrix().to_c();
    }

    pub fn on_ground(&self) -> bool {
        self.0.on_ground
    }

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
