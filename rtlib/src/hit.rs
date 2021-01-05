use std::rc::Rc;

use crate::vec3::Vec3;
use crate::{material::Blank, ray::Ray};
use crate::{material::Material, point3::Point3};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::new(),
            normal: Vec3::new(),
            mat: Rc::new(Blank::new()),
            t: 0.,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(*outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -*outward_normal,
        }
    }
}

#[derive(Default)]
pub struct HittableObjects {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableObjects {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add<H: Hittable + 'static>(&mut self, h: H) {
        self.objects.push(Rc::new(h));
    }
}

impl Hittable for HittableObjects {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for o in &self.objects {
            match o.as_ref().hit(ray, t_min, closest_so_far) {
                Some(rec) => {
                    closest_so_far = rec.t;
                    hit_record = Some(rec);
                }
                None => {}
            }
        }

        hit_record
    }
}
