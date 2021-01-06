use crate::{color::Color, hit, ray::Ray, vec3::Vec3};

pub struct Record {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, h_rec: &hit::Record) -> Option<Record>;
}

#[derive(Default)]
pub struct Blank {}

impl Blank {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for Blank {
    fn scatter(&self, _ray_in: &Ray, _h_rec: &hit::Record) -> Option<Record> {
        None
    }
}

#[derive(Default)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, h_rec: &hit::Record) -> Option<Record> {
        let mut scatter_direction = h_rec.normal + Vec3::rand_unit();

        // Catch degenerate scatter direction.
        if scatter_direction.is_near_zero() {
            scatter_direction = h_rec.normal;
        };

        Some(Record {
            scattered: Ray {
                origin: h_rec.p,
                direction: scatter_direction,
            },
            attenuation: self.albedo,
        })
    }
}

#[derive(Default)]
pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, h_rec: &hit::Record) -> Option<Record> {
        let reflected = ray_in.direction.unit().reflect(h_rec.normal);
        let scattered = Ray {
            origin: h_rec.p,
            direction: reflected,
        };
        if scattered.direction.dot(h_rec.normal) > 0. {
            Some(Record {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}
