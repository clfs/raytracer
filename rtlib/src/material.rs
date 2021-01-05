use crate::{color::Color, hit::Record, ray::Ray, vec3::Vec3};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &Record) -> Option<ScatterRecord>;
}

pub struct Blank {}

impl Blank {
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for Blank {
    fn scatter(&self, _ray_in: &Ray, _rec: &Record) -> Option<ScatterRecord> {
        None
    }
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &Record) -> Option<ScatterRecord> {
        let mut scatter_direction = rec.normal + Vec3::rand_unit();

        // Catch degenerate scatter direction.
        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal;
        };

        Some(ScatterRecord {
            scattered: Ray {
                origin: rec.p,
                direction: scatter_direction,
            },
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &Record) -> Option<ScatterRecord> {
        let reflected = ray_in.direction.unit().reflect(rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected,
        };
        if scattered.direction.dot(rec.normal) > 0. {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}
