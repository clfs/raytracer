use crate::{point3::Point3, ray::Ray, vec3::Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,

    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: &Point3,
        look_at: &Point3,
        v_up: &Vec3,
        vfov: f64, // degrees.
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (*look_from - *look_at).unit();
        let u = (*v_up).cross(w).unit();
        let v = w.cross(u);

        let origin = *look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - focus_dist * w;
        let lens_radius = aperture / 2.;

        Self {
            aspect_ratio,
            viewport_height,
            viewport_width,
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }
}

impl Camera {
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::rand_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }
}
