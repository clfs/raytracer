use super::point3::Point3;
use super::ray::Ray;

pub fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = (ray.origin - *center).to_vec3();
    let a = ray.direction.mag_squared();
    let half_b = oc.dot(ray.direction);
    let c = oc.mag_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}
