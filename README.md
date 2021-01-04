# raytracer

Rust port of a C++ ray tracer.

## Overloaded operations

I'm using a few overloaded operations. They're mostly intuitive (e.g., `Vec3`
multiplied by `f64` is `Vec3`), but some are important to know:

|      lhs |  op |      rhs |          result |
|----------|-----|----------|-----------------|
| `Point3` | `+` |   `Vec3` |        `Point3` |
| `Point3` | `-` |   `Vec3` |        `Point3` |
|   `Vec3` | `+` | `Point3` |        `Point3` |
|   `Vec3` | `-` | `Point3` |        `Point3` |
| `Point3` | `+` | `Point3` | not implemented |
| `Point3` | `-` | `Point3` |          `Vec3` |
