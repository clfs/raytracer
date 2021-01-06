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
    pub const fn new() -> Self {
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

impl Lambertian {
    pub const fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }
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
    pub fuzz: f64,
}

impl Metal {
    pub const fn new(albedo: &Color, fuzz: f64) -> Self {
        Self {
            albedo: *albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, h_rec: &hit::Record) -> Option<Record> {
        let reflected = ray_in.direction.unit().reflect(&h_rec.normal);
        let scattered = Ray {
            origin: h_rec.p,
            direction: reflected + self.fuzz * Vec3::rand_in_unit_sphere(),
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

#[derive(Default)]
pub struct Dielectric {
    pub ir: f64, // Index of refraction.
}

impl Dielectric {
    pub const fn new(ir: f64) -> Self {
        Self { ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, h_rec: &hit::Record) -> Option<Record> {
        let attenuation = Color::new(1., 1., 1.);
        let refraction_ratio = if h_rec.front_face {
            1. / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray_in.direction.unit();
        let cos_theta = -unit_direction.dot(h_rec.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let direction = if cannot_refract {
            unit_direction.reflect(&h_rec.normal)
        } else {
            unit_direction.refract(&h_rec.normal, refraction_ratio)
        };

        Some(Record {
            attenuation,
            scattered: Ray {
                origin: h_rec.p,
                direction,
            },
        })
    }
}
