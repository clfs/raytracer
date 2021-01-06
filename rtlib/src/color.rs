use std::ops;

#[derive(Clone, Copy, Default)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn to_rgb(&self, samples_per_pixel: u32) -> [u8; 3] {
        let scale = 1. / f64::from(samples_per_pixel);
        // TODO(cfiguereosupran) Eliminate these clippy allows.
        #[allow(clippy::cast_possible_truncation, clippy::clippy::cast_sign_loss)]
        [
            (256. * (self.r * scale).sqrt().max(0.).min(0.999)) as u8,
            (256. * (self.g * scale).sqrt().max(0.).min(0.999)) as u8,
            (256. * (self.b * scale).sqrt().max(0.).min(0.999)) as u8,
        ]
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
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
        Color::new(self * rhs.r, self * rhs.g, self * rhs.b)
    }
}

impl ops::Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}
