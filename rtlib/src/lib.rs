#![allow(clippy::must_use_candidate)]

pub mod camera;
pub mod color;
pub mod hit;
pub mod material;
pub mod point3;
pub mod ray;
pub mod sphere;
pub mod vec3;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
