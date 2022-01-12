use std::cmp::Ordering;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Float(f64);

impl Float {
    pub fn value(&self) -> f64 {
        self.0
    }
    pub fn new(value: f64) -> Float {
        Float(value)
    }

    pub fn sqrt(&self) -> Float {
        Float(self.value().sqrt())
    }

    pub fn to_string(&self) -> String {
        self.value().to_string()
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        (self.value().abs() - other.value().abs()).abs() <= f64::EPSILON
    }
}

impl ops::Add for Float {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.value() + other.value())
    }
}

impl ops::Sub for Float {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.value() - other.value())
    }
}

impl ops::Mul for Float {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.value() * other.value())
    }
}

impl ops::Mul<f64> for Float {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self(self.value() * other)
    }
}

impl ops::Div for Float {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self(self.value() / other.value())
    }
}

impl ops::Neg for Float {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.value())
    }
}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        if self.value() < other.value() {
            return Some(Ordering::Less);
        }

        Some(Ordering::Greater)
    }
}

#[cfg(test)]
mod float_tests {
    use super::Float;

    #[test]
    fn can_be_compared_equal() {
        let a = Float::new(1.0);
        let b = Float::new(1.0);
        let c = Float::new(1.1);

        assert_eq!(a == b, true);
        assert_eq!(a != b, false);
        assert_eq!(a == c, false);
        assert_eq!(a != c, true);
    }

    #[test]
    fn can_be_compared_less() {
        let a = Float::new(1.0);
        let b = Float::new(1.5);

        assert_eq!(a < b, true);
        assert_eq!(b < a, false);
    }

    #[test]
    fn can_be_compared_greater() {
        let a = Float::new(1.0);
        let b = Float::new(1.5);

        assert_eq!(b > a, true);
        assert_eq!(a > b, false);
    }

    #[test]
    fn can_be_compared_greater_or_equal() {
        let a = Float::new(1.0);
        let b = Float::new(1.0);

        assert_eq!(b >= a, true);
        assert_eq!(a >= b, true);
    }

    #[test]
    fn can_be_compared_less_or_equal() {
        let a = Float::new(1.0);
        let b = Float::new(1.0);

        assert_eq!(b <= a, true);
        assert_eq!(a <= b, true);
    }

    #[test]
    fn can_be_added() {
        let a = Float::new(1.0);
        let b = Float::new(1.5);

        assert_eq!(a + b, Float::new(2.5));
    }

    #[test]
    fn can_be_substracted() {
        let a = Float::new(1.0);
        let b = Float::new(1.5);

        assert_eq!(a - b, Float::new(-0.5));
    }

    #[test]
    fn can_be_multiplied() {
        let a = Float::new(2.0);
        let b = Float::new(1.5);

        assert_eq!(a * b, Float::new(3.0));
    }

    #[test]
    fn can_be_divided() {
        let a = Float::new(1.0);
        let b = Float::new(2.0);

        assert_eq!(a / b, Float::new(0.5));
    }
}
