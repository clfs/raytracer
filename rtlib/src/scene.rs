use std::rc::Rc;

use rand::{prelude::ThreadRng, Rng};

use crate::{
    color::Color,
    hit::HittableObjects,
    material::{Dielectric, Lambertian, Metal},
    point3::Point3,
    sphere::Sphere,
};

pub fn random() -> HittableObjects {
    fn rand_sphere(a: i32, b: i32, rng: &mut ThreadRng) -> Option<Sphere> {
        let choose_mat: f64 = rng.gen();
        let center = Point3::new(
            0.9_f64.mul_add(rng.gen::<f64>(), f64::from(a)),
            0.2,
            0.9_f64.mul_add(rng.gen::<f64>(), f64::from(b)),
        );

        if (center - Point3::new(4., 0.2, 0.)).mag() > 0.9 {
            if choose_mat < 0.8 {
                // diffuse
                let albedo = Color::rand() * Color::rand();
                let mat = Rc::new(Lambertian::new(&albedo));
                return Some(Sphere::new(&center, 0.2, mat));
            } else if choose_mat < 0.95 {
                // metal
                let albedo = Color::rand_in(0.5, 1.);
                let fuzz = rng.gen_range(0.0..0.5);
                let mat = Rc::new(Metal::new(&albedo, fuzz));
                return Some(Sphere::new(&center, 0.2, mat));
            } else {
                // glass
                let mat = Rc::new(Dielectric::new(1.5));
                return Some(Sphere::new(&center, 0.2, mat));
            }
        };
        None
    }

    let mut world = HittableObjects::new();

    let ground_material = Rc::new(Lambertian::new(&Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(
        &Point3::new(0., -1000., 0.),
        1000.,
        ground_material,
    ));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            if let Some(s) = rand_sphere(a, b, &mut rng) {
                world.add(s)
            }
        }
    }

    let mat1 = Rc::new(Dielectric::new(1.5));
    let mat2 = Rc::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1)));
    let mat3 = Rc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(&Point3::new(0., 1., 0.), 1., mat1);
    let sphere2 = Sphere::new(&Point3::new(-4., 1., 0.), 1., mat2);
    let sphere3 = Sphere::new(&Point3::new(4., 1., 0.), 1., mat3);

    world.add(sphere1);
    world.add(sphere2);
    world.add(sphere3);

    world
}
