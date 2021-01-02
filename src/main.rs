use std::path::Path;

use clap::Clap;
use image::ImageBuffer;

use rtlib::{
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

// Camera
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

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

    // Set up some more camera variables.
    let origin = Point3::new();
    let horizontal = Vec3 {
        x: VIEWPORT_WIDTH,
        ..Default::default()
    };
    let vertical = Vec3 {
        y: VIEWPORT_HEIGHT,
        ..Default::default()
    };
    let lower_left_corner: Point3 = (origin.to_vec3()
        - horizontal / 2.0
        - vertical / 2.0
        - Vec3 {
            z: FOCAL_LENGTH,
            ..Default::default()
        })
    .to_point3();

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // `enumerate_pixels_mut` places the origin at the top left corner, but
        // the author places the origin at the bottom left corner. Luckily, we
        // can adjust on the fly by altering the y-coordinate.
        let yy = IMAGE_HEIGHT - y - 1;

        if x == 0 {
            println!("\rScanlines remaining: {}", yy);
        }

        let u = x as f64 / (IMAGE_WIDTH - 1) as f64;
        let v = yy as f64 / (IMAGE_HEIGHT - 1) as f64;
        let r = Ray {
            origin,
            direction: lower_left_corner.to_vec3() + u * horizontal + v * vertical
                - origin.to_vec3(),
        };

        *pixel = image::Rgb(ray_color(&r, &world).to_rgb())
    }

    imgbuf
        .save(&opts.filename)
        .expect("failed to write to file")
}

fn ray_color(ray: &Ray, world: &HittableObjects) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(&ray, 0.0, std::f64::INFINITY, &mut rec) {
        return 0.5
            * (rec.normal.to_color()
                + Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                });
    };
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
