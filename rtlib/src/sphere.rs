use crate::hit::{HitRecord, Hittable};
use crate::point3::Point3;
use crate::ray::Ray;

#[derive(Default)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Compute the discriminant.
        let oc = ray.origin - self.center;
        let a = ray.direction.mag_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.mag_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        // Pick a root that lies in the acceptable range, if possible.
        let sqrtd = discriminant.sqrt();
        let sub_root = (-half_b - sqrtd) / a;
        let add_root = (-half_b + sqrtd) / a;
        let root: Option<f64> = if t_min < sub_root && sub_root < t_max {
            Some(sub_root)
        } else if t_min < add_root && add_root < t_max {
            Some(add_root)
        } else {
            None
        };

        // Update the HitRecord.
        match root {
            Some(v) => rec.t = v,
            None => return false, // No hit.
        }
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        true
    }
}
