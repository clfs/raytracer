use std::path::Path;

use clap::Clap;
use image::ImageBuffer;

use rtlib::color::Color;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

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

    let mut imgbuf = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb(
            Color {
                r: x as f64 / ((IMAGE_WIDTH - 1) as f64),
                g: y as f64 / ((IMAGE_HEIGHT - 1) as f64),
                b: 0.25,
            }
            .to_rgb(),
        )
    }

    // There's a silly race condition here, where another process might create
    // the file _after_ the no-exist check, but _before_ the write attempt. This
    // is avoidable by using `std::fs::OpenOptions`, but I couldn't figure out
    // how to serialize an `ImageBuffer` to raw bytes (for using File methods).
    // Instead, I'm just using the provided (dangerous) `save` method, which can
    // overwrite files. I should change this at some point.
    match Path::new(&opts.filename).exists() {
        true => panic!("file already exists"),
        false => imgbuf
            .save(&opts.filename)
            .expect("failed to write to file"),
    }
}
