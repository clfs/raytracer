use std::path::Path;

use clap::Clap;
use image::ImageBuffer;
use rand::Rng;

use rtlib::{
    camera::Camera,
    color::Color,
    hit::{HitRecord, Hittable, HittableObjects},
    point3::Point3,
    ray::Ray,
    sphere::Sphere,
    vec3::Vec3,
};

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
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
    world.add(Sphere {
        center: Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    });
    world.add(Sphere {
        center: Point3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
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

        let mut color = Color::new();
        for _ in 0..SAMPLES_PER_PIXEL {
            let u = (x as f64 + rng.gen::<f64>()) / ((IMAGE_WIDTH - 1) as f64);
            let v = (yy as f64 + rng.gen::<f64>()) / ((IMAGE_HEIGHT - 1) as f64);
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
        return Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
    }

    let mut rec = HitRecord::new();

    if world.hit(&ray, 0.0, std::f64::INFINITY, &mut rec) {
        let target: Point3 =
            rec.p + rec.normal.to_point3() + Vec3::rand_in_unit_sphere().to_point3();
        return 0.5
            * ray_color(
                &Ray {
                    origin: rec.p,
                    direction: (target - rec.p).to_vec3(),
                },
                &world,
                depth - 1,
            );
    }

    let t = 0.5 * (ray.direction.unit().y + 1.0);
    return (1.0 - t)
        * Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
        + t * Color {
            r: 0.5,
            g: 0.7,
            b: 1.0,
        };
}
