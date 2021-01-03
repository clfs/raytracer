use std::ops;

use crate::vec3::Vec3;

#[derive(Clone, Copy, Default)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 {
    pub fn zero() -> Self {
        Default::default()
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl ops::Add for Point3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Point3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl ops::Sub for Point3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign for Point3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl ops::Div<f64> for Point3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
