use std::ops;

use rand::Rng;

use crate::point3::Point3;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn zero() -> Self {
        Default::default()
    }

    // Exclusive, i.e. cannot return <1.0, 1.0, 1.0>.
    pub fn rand_in_unit_sphere() -> Self {
        let mut rng = rand::thread_rng();
        let mut v = Vec3::new();
        loop {
            v.x = rng.gen_range(-1.0..1.0);
            v.y = rng.gen_range(-1.0..1.0);
            v.z = rng.gen_range(-1.0..1.0);
            if v.mag_squared() < 1.0 {
                return v;
            }
        }
    }

    // Exclusive, i.e. cannot return <1.0, 1.0, 1.0>.
    pub fn rand_in_unit_hemisphere(normal: &Vec3) -> Self {
        let v = Vec3::rand_in_unit_sphere();
        match v.dot(*normal) > 0.0 {
            true => v,
            false => -v,
        }
    }

    pub fn rand_unit() -> Self {
        Vec3::rand_in_unit_sphere().unit()
    }

    pub fn is_near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(&self, normal: Vec3) -> Self {
        *self - 2.0 * self.dot(normal) * normal
    }

    pub fn mag(self) -> f64 {
        self.mag_squared().sqrt()
    }

    pub fn mag_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit(&self) -> Self {
        *self / self.mag()
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<Point3> for Vec3 {
    type Output = Point3;

    fn add(self, rhs: Point3) -> Self::Output {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub<Point3> for Vec3 {
    type Output = Point3;

    fn sub(self, rhs: Point3) -> Self::Output {
        Point3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
