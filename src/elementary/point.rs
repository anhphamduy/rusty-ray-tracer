use crate::elementary::float::Float;
use crate::elementary::tuple::Tuple;
use crate::elementary::vector::Vector;

use std::ops;

#[derive(Debug)]
pub struct Point(Tuple);

impl Point {
    pub fn new(x: Float, y: Float, z: Float) -> Point {
        Point(Tuple::new(x, y, z, Float::new(1.0)).unwrap())
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
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl ops::Sub for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Vector {
        let result = (self.0 - other.0).unwrap();

        Vector::new(result.x(), result.y(), result.z())
    }
}

impl ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, other: Vector) -> Point {
        Point::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

#[cfg(test)]
mod point_tests {
    use super::Float;
    use super::Point;
    use super::Vector;

    #[test]
    fn can_substract_two_points_to_a_vector() {
        let a = Point::new(Float::new(3.0), Float::new(2.0), Float::new(1.0));
        let b = Point::new(Float::new(5.0), Float::new(6.0), Float::new(7.0));

        assert_eq!(
            a - b,
            Vector::new(Float::new(-2.0), Float::new(-4.0), Float::new(-6.0)),
        );
    }

    #[test]
    fn can_substract_vector_from_a_point() {
        let a = Point::new(Float::new(3.0), Float::new(2.0), Float::new(1.0));
        let b = Vector::new(Float::new(5.0), Float::new(6.0), Float::new(7.0));

        assert_eq!(
            a - b,
            Point::new(Float::new(-2.0), Float::new(-4.0), Float::new(-6.0)),
        )
    }
}
