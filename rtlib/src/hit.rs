use super::point3::Point3;
use super::ray::Ray;

pub fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let oc = (ray.origin - *center).to_vec3();
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}
