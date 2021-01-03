use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,

    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    // The author uses defaults. I'm unsure if we'll change these values later,
    // so this is messy at least for now.
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::zero();
        let horizontal = Vec3 {
            x: viewport_width,
            ..Default::default()
        };
        let vertical = Vec3 {
            y: viewport_height,
            ..Default::default()
        };
        let lower_left_corner = (origin.to_vec3()
            - horizontal / 2.0
            - vertical / 2.0
            - Vec3 {
                z: focal_length,
                ..Default::default()
            })
        .to_point3();

        Self {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner.to_vec3() + u * self.horizontal + v * self.vertical
                - self.origin.to_vec3(),
        }
    }
}