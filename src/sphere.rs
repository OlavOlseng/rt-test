use crate::vec3;
use crate::vec3::*;
use crate::ray::*;


pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
}

impl Sphere {

    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius
        }
    }
}

impl Hitable for Sphere {
    fn is_hit_by(&self, ray: &Ray) -> f32 {
        let oc = ray.origin - self.center;
        let a = vec3::dot(&ray.direction, &ray.direction);
        let b = 2.0 * vec3::dot(&oc, &ray.direction);
        let c = vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b*b - 4.0*a*c;
        if discriminant < 0.0 {
            -1.0
        } else {
            -b  - discriminant.sqrt() / 2.0 / a
        }
    }
}