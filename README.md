# raytracer

A Rust port of an existing [C++ ray tracer](https://raytracing.github.io/books/RayTracingInOneWeekend.html).
Here's a demo image:

![A scene made from raytraced spheres](./out.png)

## Usage

Use the `-f` flag to provide an output filename. It'll deduce the image format
from the file extension. It'll also refuse to overwrite an existing file.

```text
cargo run --release -- -f filename.png
cargo run --release -- -f filename.jpg
etc.
```

## Raytracer features

- Anti-aliasing
- Defocus blur
- Dielectric materials
- Diffuse materials
- Fuzzy reflection
- Gamma correction
- Lambertian reflection
- Light scatter and reflectance
- Linear gradients
- Multiple image output formats
- Positionable and orientable camera
- Progress indicators
- Proper internal reflection
- Shadow acne removal
- Thin-lens approximation
- Variable field-of-view

## Overloaded operations

I'm using a few overloaded operations. They're mostly intuitive (e.g., `Vec3`
multiplied by `f64` is `Vec3`), but some common ones are important to know:

|      lhs |  op |      rhs |          result |
|----------|-----|----------|-----------------|
| `Point3` | `+` |   `Vec3` |        `Point3` |
| `Point3` | `-` |   `Vec3` |        `Point3` |
|   `Vec3` | `+` | `Point3` |        `Point3` |
|   `Vec3` | `-` | `Point3` |        `Point3` |
| `Point3` | `+` | `Point3` | not implemented |
| `Point3` | `-` | `Point3` |          `Vec3` |
|   `Vec3` | `*` |   `Vec3` | not implemented |
| `Color3` | `*` | `Color3` |        `Color3` |
