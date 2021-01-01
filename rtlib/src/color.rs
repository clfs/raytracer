#[derive(Default)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        [
            (self.r * 255.999) as u8,
            (self.g * 255.999) as u8,
            (self.b * 255.999) as u8,
        ]
    }
}
