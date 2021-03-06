use std::path::Path;

use clap::Clap;
use image::ImageBuffer;
use rand::Rng;

use rtlib::{
    camera::Camera,
    color::Color,
    hit::{Hittable, HittableObjects},
    point3::Point3,
    ray::Ray,
    scene,
    vec3::Vec3,
};

// Image
const IMAGE_WIDTH: u32 = 1200;
const IMAGE_HEIGHT: u32 = 800;
const SAMPLES_PER_PIXEL: u32 = 500;
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
    let world = scene::random();

    // Set up camera.
    let look_from = Point3::new(13., 2., 3.);
    let look_at = Point3::new(0., 0., 0.);
    let v_up = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let aspect_ratio = f64::from(IMAGE_WIDTH) / f64::from(IMAGE_HEIGHT);
    let vfov: f64 = 20.; // degrees

    let camera = Camera::new(
        &look_from,
        &look_at,
        &v_up,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

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
            let v = (f64::from(yy) + rng.gen::<f64>()) / f64::from(IMAGE_HEIGHT - 1);
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

    world.hit(ray, 0.001, std::f64::INFINITY).map_or_else(
        || {
            let t = 0.5 * (ray.direction.unit().y + 1.);
            (1. - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.)
        },
        |h_rec| {
            h_rec.mat.scatter(ray, &h_rec).map_or_else(
                || Color::new(0., 0., 0.),
                |s_rec| s_rec.attenuation * ray_color(&s_rec.scattered, world, depth - 1),
            )
        },
    )
}
