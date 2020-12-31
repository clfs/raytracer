use std::path::Path;

use clap::Clap;
use image::ImageBuffer;

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
        let r = x as f64 / ((IMAGE_WIDTH - 1) as f64);
        let g = y as f64 / ((IMAGE_HEIGHT - 1) as f64);
        let b = 0.25;

        let cr = (r * 255.999) as u8;
        let cg = (g * 255.999) as u8;
        let cb = (b * 255.999) as u8;

        *pixel = image::Rgb([cr, cg, cb]);
    }

    // There's a silly race condition here, where the file might exist _after_
    // the no-exist check, but _before_ the write attempt. This is avoidable by
    // using `std::fs::OpenOptions`, but I couldn't figure out how to serialize
    // an `ImageBuffer` to raw bytes (for writing). Instead, I'm just using the
    // provided (dangerous) `save` method.
    match Path::new(&opts.filename).exists() {
        true => panic!("file already exists"),
        false => imgbuf.save(&opts.filename).expect("failed to write to file"),
    }
}
