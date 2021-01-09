use std::rc::Rc;

use crate::{
    material::{Blank, Material},
    point3::Point3,
    ray::Ray,
    vec3::Vec3,
};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Record>;
}

#[derive(Clone)]
pub struct Record {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl Default for Record {
    fn default() -> Self {
        Self::new()
    }
}

impl Record {
    pub fn new() -> Self {
        Self {
            p: Point3::zero(),
            normal: Vec3::zero(),
            mat: Rc::new(Blank::new()),
            t: 0.,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(*outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

#[derive(Default)]
pub struct HittableObjects {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableObjects {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add<H: Hittable + 'static>(&mut self, h: H) {
        self.objects.push(Rc::new(h));
    }
}

impl Hittable for HittableObjects {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Record> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for o in &self.objects {
            if let Some(rec) = o.as_ref().hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }

        hit_record
    }
}
