use std::rc::Rc;

use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::{
    hit::{Hittable, Record},
    material::Blank,
};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Rc<dyn Material>,
}

impl Sphere {
    #[must_use]
    pub fn new() -> Self {
        Self {
            center: Point3::zero(),
            radius: 0.,
            mat: Rc::new(Blank::new()),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Record> {
        // Compute the discriminant.
        let oc = ray.origin - self.center;
        let a = ray.direction.mag_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.mag_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
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
        let mut record = Record::new();
        match root {
            Some(v) => record.t = v,
            None => return None, // No hit.
        }
        record.p = ray.at(record.t);
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(ray, &outward_normal);
        record.mat = self.mat.clone();

        Some(record)
    }
}
