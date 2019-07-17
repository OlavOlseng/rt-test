use crate::vec3;
use crate::vec3::*;
use crate::ray::*;
use crate::hit::*;


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
    fn is_hit_by(&self, t_min: f32, t_max: f32, ray: &Ray) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = vec3::dot(&ray.direction, &ray.direction);
        let b = vec3::dot(&oc, &ray.direction);
        let c = vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
           
           let mut t = (-b - discriminant.sqrt()) / a;
           if t < t_max && t > t_min {
               let hit_point = ray.point_at_time_t(t);
               return Some(Hit::new(
                   hit_point,
                   (hit_point - self.center) / self.radius,
                   t
               ))
            }

            t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let hit_point = ray.point_at_time_t(t);                
                return Some(Hit::new(
                   hit_point,
                   (hit_point - self.center) / self.radius,
                   t
               ))
            }
        }

        None
    }
}