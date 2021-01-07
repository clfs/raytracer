use std::rc::Rc;

use rand::{prelude::ThreadRng, Rng};

use crate::{
    color::Color,
    hit::HittableObjects,
    material::{Dielectric, Lambertian, Metal},
    point3::Point3,
    sphere::Sphere,
};

/*hittable_list random_scene() {
    hittable_list world;

    auto ground_material = make_shared<lambertian>(color(0.5, 0.5, 0.5));
    world.add(make_shared<sphere>(point3(0,-1000,0), 1000, ground_material));

    for (int a = -11; a < 11; a++) {
        for (int b = -11; b < 11; b++) {
            auto choose_mat = random_double();
            point3 center(a + 0.9*random_double(), 0.2, b + 0.9*random_double());

            if ((center - point3(4, 0.2, 0)).length() > 0.9) {
                shared_ptr<material> sphere_material;

                if (choose_mat < 0.8) {
                    // diffuse
                    auto albedo = color::random() * color::random();
                    sphere_material = make_shared<lambertian>(albedo);
                    world.add(make_shared<sphere>(center, 0.2, sphere_material));
                } else if (choose_mat < 0.95) {
                    // metal
                    auto albedo = color::random(0.5, 1);
                    auto fuzz = random_double(0, 0.5);
                    sphere_material = make_shared<metal>(albedo, fuzz);
                    world.add(make_shared<sphere>(center, 0.2, sphere_material));
                } else {
                    // glass
                    sphere_material = make_shared<dielectric>(1.5);
                    world.add(make_shared<sphere>(center, 0.2, sphere_material));
                }
            }
        }
    }

    auto material1 = make_shared<dielectric>(1.5);
    world.add(make_shared<sphere>(point3(0, 1, 0), 1.0, material1));

    auto material2 = make_shared<lambertian>(color(0.4, 0.2, 0.1));
    world.add(make_shared<sphere>(point3(-4, 1, 0), 1.0, material2));

    auto material3 = make_shared<metal>(color(0.7, 0.6, 0.5), 0.0);
    world.add(make_shared<sphere>(point3(4, 1, 0), 1.0, material3));

    return world;
} */

pub fn random_scene() -> HittableObjects {
    let mut world = HittableObjects::new();

    let ground_material = Rc::new(Lambertian::new(&Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(
        &Point3::new(0., -1000., 0.),
        1000.,
        ground_material,
    ));

    let mut rng = rand::thread_rng();

    fn rand_sphere(a: i32, b: i32, rng: &mut ThreadRng) -> Option<Sphere> {
        let choose_mat: f64 = rng.gen();
        let center = Point3::new(
            a as f64 + 0.9 * rng.gen::<f64>(),
            0.2,
            b as f64 + 0.9 * rng.gen::<f64>(),
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

    for a in -11..11 {
        for b in -11..11 {
            match rand_sphere(a, b, &mut rng) {
                Some(s) => world.add(s),
                None => {}
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
