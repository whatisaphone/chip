use na::{Matrix3, Point3, Real, Vector3};

pub trait ToFromC<T> {
    fn from_c(T) -> Self;
    fn to_c(&self) -> T;
}

impl<N: Real> ToFromC<[N; 3]> for Vector3<N> {
    fn from_c(x: [N; 3]) -> Self {
        Vector3::new(x[0], x[1], x[2])
    }

    fn to_c(&self) -> [N; 3] {
        [self.x, self.y, self.z]
    }
}

impl<N: Real> ToFromC<[N; 3]> for Point3<N> {
    fn from_c(x: [N; 3]) -> Self {
        Point3::new(x[0], x[1], x[2])
    }

    fn to_c(&self) -> [N; 3] {
        [self.x, self.y, self.z]
    }
}

impl<N: Real> ToFromC<[N; 9]> for Matrix3<N> {
    fn from_c(x: [N; 9]) -> Self {
        Matrix3::new(x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7], x[8])
    }

    fn to_c(&self) -> [N; 9] {
        [
            self[(0, 0)],
            self[(0, 1)],
            self[(0, 2)],
            self[(1, 0)],
            self[(1, 1)],
            self[(1, 2)],
            self[(2, 0)],
            self[(2, 1)],
            self[(2, 2)],
        ]
    }
}

#[cfg(test)]
mod tests {
    use extend::ToFromC;
    use na::{Matrix3, Point3, Vector3};

    #[test]
    fn vector3_to_c() {
        let x = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(x.to_c(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn vector3_from_c() {
        let x = [1.0, 2.0, 3.0];
        assert_eq!(Vector3::from_c(x), Vector3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn point3_to_c() {
        let x = Point3::new(1.0, 2.0, 3.0);
        assert_eq!(x.to_c(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn point3_from_c() {
        let x = [1.0, 2.0, 3.0];
        assert_eq!(Point3::from_c(x), Point3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn matrix3_to_c() {
        let x = Matrix3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(x.to_c(), [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    }

    #[test]
    fn matrix3_from_c() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let expected = Matrix3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(Matrix3::from_c(x), expected);
    }
}
