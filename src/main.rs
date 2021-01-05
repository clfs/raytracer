use std::path::Path;
use std::rc::Rc;

use clap::Clap;
use image::ImageBuffer;
use rand::Rng;

use rtlib::{
    camera::Camera,
    color::Color,
    hit::{Hittable, HittableObjects},
    material::{Lambertian, Metal},
    point3::Point3,
    ray::Ray,
    sphere::Sphere,
};

// Image
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = 225;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;

#[derive(Clap)]
#[clap(
    version = "0.1.0",
    author = "Calvin Figuereo-Supraner <mail@calvin.page>"
)]
struct Opts {
    #[clap(short, long)]
    filename: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    // There's a silly race condition here, where another process might create
    // the file _after_ the no-exist check, but _before_ the write attempt. This
    // is avoidable by using `std::fs::OpenOptions`, but I couldn't figure out
    // how to serialize an `ImageBuffer` to raw bytes (for using File methods).
    // Instead, I'm just using the provided (dangerous) `save` method, which can
    // overwrite files. I should change this at some point.
    if Path::new(&opts.filename).exists() {
        panic!("file already exists")
    }

    let mut imgbuf = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // Set up the world.
    let mut world = HittableObjects::new();

    let material_ground = Rc::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.),
    });
    let material_center = Rc::new(Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    });
    let material_left = Rc::new(Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
    });
    let material_right = Rc::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
    });

    world.add(Sphere {
        center: Point3::new(0., -100.5, -1.),
        radius: 100.,
        mat: material_ground,
    });
    world.add(Sphere {
        center: Point3::new(0., 0., -1.),
        radius: 0.5,
        mat: material_center,
    });
    world.add(Sphere {
        center: Point3::new(-1., 0., -1.),
        radius: 0.5,
        mat: material_left,
    });
    world.add(Sphere {
        center: Point3::new(1., 0., -1.),
        radius: 0.5,
        mat: material_right,
    });

    let camera = Camera::new();

    let mut rng = rand::thread_rng();

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // `enumerate_pixels_mut` places the origin at the top left corner, but
        // the author places the origin at the bottom left corner. Luckily, we
        // can adjust on the fly by altering the y-coordinate.
        let yy = IMAGE_HEIGHT - y - 1;

        if x == 0 {
            println!("Scanlines remaining: {}", yy);
        }

        let mut color = Color::default();
        for _ in 0..SAMPLES_PER_PIXEL {
            let u = (f64::from(x) + rng.gen::<f64>()) / f64::from(IMAGE_WIDTH - 1);
            let v = (f64::from(yy) as f64 + rng.gen::<f64>()) / f64::from(IMAGE_HEIGHT - 1);
            let ray = camera.get_ray(u, v);
            color += ray_color(&ray, &world, MAX_DEPTH);
        }

        *pixel = image::Rgb(color.to_rgb(SAMPLES_PER_PIXEL))
    }

    imgbuf
        .save(&opts.filename)
        .expect("failed to write to file")
}

fn ray_color(ray: &Ray, world: &HittableObjects, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0., 0., 0.);
    }

    if let Some(rec) = world.hit(&ray, 0.001, std::f64::INFINITY) {
        let mut scattered = Ray::new();
        let mut attenuation = Color::default();
        if rec
            .mat
            .scatter(&ray, &rec, &mut attenuation, &mut scattered)
        {
            attenuation * ray_color(&scattered, &world, depth - 1)
        } else {
            Color::new(0., 0., 0.)
        }
    } else {
        let t = 0.5 * (ray.direction.unit().y + 1.);
        (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
    }
}
