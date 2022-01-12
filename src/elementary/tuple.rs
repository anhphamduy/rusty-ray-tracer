use crate::elementary::float::Float;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Tuple(Float, Float, Float, Float);

impl Tuple {
    pub fn new(x: Float, y: Float, z: Float, w: Float) -> Result<Tuple, String> {
        if w != Float::new(0.0) && w != Float::new(1.0) {
            return Err(String::from("w must be either 1.0 or 0.0"));
        }

        Ok(Tuple(x, y, z, w))
    }

    pub fn x(&self) -> Float {
        self.0
    }

    pub fn y(&self) -> Float {
        self.1
    }

    pub fn z(&self) -> Float {
        self.2
    }

    pub fn w(&self) -> Float {
        self.3
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
    }
}

impl ops::Add for Tuple {
    type Output = Result<Self, String>;

    fn add(self, other: Self) -> Result<Self, String> {
        Tuple::new(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}

impl ops::Sub for Tuple {
    type Output = Result<Self, String>;

    fn sub(self, other: Self) -> Result<Self, String> {
        if self.3 < other.3 {
            return Tuple::new(
                self.0 - other.0,
                self.1 - other.1,
                self.2 - other.2,
                Float::new(1.0),
            );
        }

        Tuple::new(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
        )
    }
}

#[cfg(test)]
mod tuple_tests {
    use super::Float;
    use super::Tuple;

    #[test]
    fn cannot_initialize_with_invalid_w() {
        let a = Tuple::new(
            Float::new(1.0),
            Float::new(1.0),
            Float::new(1.0),
            Float::new(0.5),
        );
        assert_eq!(a.is_err(), true);
    }

    #[test]
    fn cannot_initialize_with_valid_w() {
        let a = Tuple::new(
            Float::new(1.0),
            Float::new(1.0),
            Float::new(1.0),
            Float::new(1.0),
        );
        assert_eq!(a.is_ok(), true);

        let b = Tuple::new(
            Float::new(1.0),
            Float::new(1.0),
            Float::new(1.0),
            Float::new(0.0),
        );
        assert_eq!(b.is_ok(), true);
    }

    #[test]
    fn can_add_tuples() {
        let a = Tuple::new(
            Float::new(3.0),
            Float::new(-2.0),
            Float::new(5.0),
            Float::new(1.0),
        )
        .unwrap();
        let b = Tuple::new(
            Float::new(-2.0),
            Float::new(3.0),
            Float::new(1.0),
            Float::new(0.0),
        )
        .unwrap();

        assert_eq!(
            (a + b).unwrap()
                == Tuple::new(
                    Float::new(1.0),
                    Float::new(1.0),
                    Float::new(6.0),
                    Float::new(1.0),
                )
                .unwrap(),
            true
        );
    }

    #[test]
    fn can_substract_two_points() {
        let a = Tuple::new(
            Float::new(3.0),
            Float::new(2.0),
            Float::new(1.0),
            Float::new(1.0),
        )
        .unwrap();
        let b = Tuple::new(
            Float::new(5.0),
            Float::new(6.0),
            Float::new(7.0),
            Float::new(1.0),
        )
        .unwrap();

        assert_eq!(
            (a - b).unwrap(),
            Tuple::new(
                Float::new(-2.0),
                Float::new(-4.0),
                Float::new(-6.0),
                Float::new(0.0),
            )
            .unwrap()
        );
    }

    #[test]
    fn can_substract_a_vector_from_a_point() {
        let point = Tuple::new(
            Float::new(3.0),
            Float::new(2.0),
            Float::new(1.0),
            Float::new(1.0),
        )
        .unwrap();
        let vector = Tuple::new(
            Float::new(5.0),
            Float::new(6.0),
            Float::new(7.0),
            Float::new(0.0),
        )
        .unwrap();

        assert_eq!(
            (vector - point).unwrap(),
            Tuple::new(
                Float::new(-2.0),
                Float::new(-4.0),
                Float::new(-6.0),
                Float::new(1.0),
            )
            .unwrap()
        );
    }

    #[test]
    fn can_substract_two_vectors() {
        let a = Tuple::new(
            Float::new(3.0),
            Float::new(2.0),
            Float::new(1.0),
            Float::new(0.0),
        )
        .unwrap();
        let b = Tuple::new(
            Float::new(5.0),
            Float::new(6.0),
            Float::new(7.0),
            Float::new(0.0),
        )
        .unwrap();

        assert_eq!(
            (a - b).unwrap(),
            Tuple::new(
                Float::new(-2.0),
                Float::new(-4.0),
                Float::new(-6.0),
                Float::new(0.0),
            )
            .unwrap()
        );
    }
}
