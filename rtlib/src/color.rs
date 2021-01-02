use std::ops;

#[derive(Clone, Copy, Default)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn to_rgb(&self, samples_per_pixel: u32) -> [u8; 3] {
        // std::f64::clamp is nightly-only :(
        fn clamp(x: f64, min: f64, max: f64) -> f64 {
            if x < min {
                min
            } else if x > max {
                max
            } else {
                x
            }
        }

        // I'm also not a fan of how the author's set this up. A good TODO item
        // would be finding a better way to express this.
        let scale = 1.0 / samples_per_pixel as f64;
        [
            (256.0 * clamp(self.r * scale, 0.0, 0.999)) as u8,
            (256.0 * clamp(self.g * scale, 0.0, 0.999)) as u8,
            (256.0 * clamp(self.b * scale, 0.0, 0.999)) as u8,
        ]
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}
