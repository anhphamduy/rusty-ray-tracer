use crate::elementary::float::Float;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Color(Float, Float, Float);

#[derive(Debug, Clone, Copy)]
pub struct Color255(Float, Float, Float);

impl Color {
    pub fn new(r: Float, g: Float, b: Float) -> Color {
        Color(r, g, b)
    }

    pub fn r(&self) -> Float {
        self.0
    }

    pub fn g(&self) -> Float {
        self.1
    }

    pub fn b(&self) -> Float {
        self.2
    }

    pub fn to_255(&self) -> String {
        let mut r_clamped = self.r();
        if r_clamped < Float::new(0.0) {
            r_clamped = Float::new(0.0);
        } else if r_clamped > Float::new(1.0) {
            r_clamped = Float::new(1.0);
        }
        r_clamped = r_clamped * Float::new(255.0);

        let mut g_clamped = self.g();
        if g_clamped < Float::new(0.0) {
            g_clamped = Float::new(0.0);
        } else if g_clamped > Float::new(1.0) {
            g_clamped = Float::new(1.0);
        }
        g_clamped = g_clamped * Float::new(255.0);

        let mut b_clamped = self.b();
        if b_clamped < Float::new(0.0) {
            b_clamped = Float::new(0.0);
        } else if b_clamped > Float::new(1.0) {
            b_clamped = Float::new(1.0);
        }
        b_clamped = b_clamped * Float::new(255.0);

        format!(
            "{} {} {}",
            (r_clamped.value() as i64).to_string(),
            (g_clamped.value() as i64).to_string(),
            (b_clamped.value() as i64).to_string()
        )
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r() == other.r() && self.g() == other.g() && self.b() == other.b()
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color(
            self.r() + other.r(),
            self.g() + other.g(),
            self.b() + other.b(),
        )
    }
}

impl ops::Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color(
            self.r() - other.r(),
            self.g() - other.g(),
            self.b() - other.b(),
        )
    }
}

impl ops::Mul<Float> for Color {
    type Output = Color;
    fn mul(self, other: Float) -> Color {
        Color(self.r() * other, self.g() * other, self.b() * other)
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color(
            self.r() * other.r(),
            self.g() * other.g(),
            self.b() * other.b(),
        )
    }
}

#[cfg(test)]
mod color_tests {
    use super::Color;
    use super::Float;

    #[test]
    fn can_add_colors() {
        let a = Color::new(Float::new(0.9), Float::new(0.6), Float::new(0.75));
        let b = Color::new(Float::new(0.7), Float::new(0.1), Float::new(0.25));

        assert_eq!(
            a + b,
            Color::new(Float::new(1.6), Float::new(0.7), Float::new(1.0))
        );
    }

    #[test]
    fn can_subtract_colors() {
        let a = Color::new(Float::new(0.9), Float::new(0.6), Float::new(0.75));
        let b = Color::new(Float::new(0.7), Float::new(0.1), Float::new(0.25));

        assert_eq!(
            a - b,
            Color::new(Float::new(0.2), Float::new(0.5), Float::new(0.5))
        );
    }

    #[test]
    fn can_be_multiplied_by_a_scalar() {
        let a = Color::new(Float::new(0.2), Float::new(0.3), Float::new(0.4));

        assert_eq!(
            a * Float::new(2.0),
            Color::new(Float::new(0.4), Float::new(0.6), Float::new(0.8))
        );
    }

    #[test]
    fn can_be_multiplied_with_another_color() {
        let a = Color::new(Float::new(1.0), Float::new(0.2), Float::new(0.4));
        let b = Color::new(Float::new(0.9), Float::new(1.0), Float::new(0.1));

        assert_eq!(
            a * b,
            Color::new(Float::new(0.9), Float::new(0.2), Float::new(0.04))
        );
    }
}
