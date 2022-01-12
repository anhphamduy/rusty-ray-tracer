use crate::elementary::float::Float;
use crate::elementary::tuple::Tuple;
use std::num;

use std::ops;

#[derive(Debug)]
pub struct Vector(Tuple);

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Vector {
    pub fn new(x: Float, y: Float, z: Float) -> Vector {
        Vector(Tuple::new(x, y, z, Float::new(0.0)).unwrap())
    }
    pub fn x(&self) -> Float {
        self.0.x()
    }

    pub fn y(&self) -> Float {
        self.0.y()
    }

    pub fn z(&self) -> Float {
        self.0.z()
    }

    pub fn magnitude(&self) -> Float {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let magnitude = self.magnitude();
        Vector::new(
            self.x() / magnitude,
            self.y() / magnitude,
            self.z() / magnitude,
        )
    }

    pub fn dot(&self, other: Self) -> Float {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Self) -> Vector {
        let result = (self.0 + other.0).unwrap();
        Vector(result)
    }
}

impl ops::Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let result = (self.0 - other.0).unwrap();
        Vector(result)
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector::new(self.x() * other, self.y() * other, self.z() * other)
    }
}

impl ops::Mul for Vector {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }
}

#[cfg(test)]
mod vector_tests {
    use super::Float;
    use super::Vector;

    #[test]
    fn can_add_vectors() {
        let a = Vector::new(Float::new(1.0), Float::new(2.0), Float::new(3.0));
        let b = Vector::new(Float::new(2.0), Float::new(3.0), Float::new(4.0));
        assert_eq!(
            a + b,
            Vector::new(Float::new(3.0), Float::new(5.0), Float::new(7.0),)
        )
    }

    #[test]
    fn can_substract_vectors() {
        let a = Vector::new(Float::new(1.0), Float::new(2.0), Float::new(3.0));
        let b = Vector::new(Float::new(2.0), Float::new(3.0), Float::new(4.0));
        assert_eq!(
            a - b,
            Vector::new(Float::new(-1.0), Float::new(-1.0), Float::new(-1.0),)
        )
    }

    #[test]
    fn can_substract_a_vector_from_a_zero_vector() {
        let a = Vector::new(Float::new(0.0), Float::new(0.0), Float::new(0.0));
        let b = Vector::new(Float::new(1.0), Float::new(-2.0), Float::new(3.0));

        assert_eq!(
            a - b,
            Vector::new(Float::new(-1.0), Float::new(2.0), Float::new(-3.0),)
        )
    }

    #[test]
    fn can_multiply_a_vector_with_a_scalar() {
        let a = Vector::new(Float::new(1.0), Float::new(-2.0), Float::new(3.0));

        assert_eq!(
            a * 3.5,
            Vector::new(Float::new(3.5), Float::new(-7.0), Float::new(10.5),)
        )
    }

    #[test]
    fn can_compute_the_magnitude() {
        let a = Vector::new(Float::new(1.0), Float::new(0.0), Float::new(0.0));
        assert_eq!(a.magnitude(), Float::new(1.0));

        let b = Vector::new(Float::new(0.0), Float::new(1.0), Float::new(0.0));
        assert_eq!(b.magnitude(), Float::new(1.0));

        let c = Vector::new(Float::new(0.0), Float::new(0.0), Float::new(1.0));
        assert_eq!(c.magnitude(), Float::new(1.0));

        let d = Vector::new(Float::new(1.0), Float::new(2.0), Float::new(3.0));
        assert_eq!(d.magnitude(), Float::new(14.0).sqrt());

        let e = Vector::new(Float::new(-1.0), Float::new(-2.0), Float::new(-3.0));
        assert_eq!(e.magnitude(), Float::new(14.0).sqrt());
    }

    #[test]
    fn can_normalize_vectors() {
        let a = Vector::new(Float::new(4.0), Float::new(0.0), Float::new(0.0));
        assert_eq!(
            a.normalize(),
            Vector::new(Float::new(1.0), Float::new(0.0), Float::new(0.0))
        );

        let b = Vector::new(Float::new(1.0), Float::new(2.0), Float::new(3.0));
        assert_eq!(
            b.normalize(),
            Vector::new(
                Float::new(1.0) / Float::new(14.0).sqrt(),
                Float::new(2.0) / Float::new(14.0).sqrt(),
                Float::new(3.0) / Float::new(14.0).sqrt()
            )
        );

        assert_eq!(b.normalize().magnitude(), Float::new(1.0),);
    }

    #[test]
    fn can_calculate_dot_product() {
        let a = Vector::new(Float::new(1.0), Float::new(2.0), Float::new(3.0));
        let b = Vector::new(Float::new(2.0), Float::new(3.0), Float::new(4.0));
        assert_eq!(a.dot(b), Float::new(20.0));
    }

    #[test]
    fn can_calculate_cross_product() {
        let a = Vector::new(Float::new(1.0), Float::new(2.0), Float::new(3.0));
        let b = Vector::new(Float::new(2.0), Float::new(3.0), Float::new(4.0));
        assert_eq!(
            a * b,
            Vector::new(Float::new(-1.0), Float::new(2.0), Float::new(-1.0))
        );
        
        let a = Vector::new(Float::new(1.0), Float::new(2.0), Float::new(3.0));
        let b = Vector::new(Float::new(2.0), Float::new(3.0), Float::new(4.0));
        assert_eq!(
            b * a,
            Vector::new(Float::new(1.0), Float::new(-2.0), Float::new(1.0))
        )
    }
}
