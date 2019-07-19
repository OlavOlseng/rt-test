use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hit::Hit;
use rand::prelude::*;

pub trait Material {
    fn scatter(&self, hit: &Hit, t_min: f32, t_max: f32, rng: &mut ThreadRng) -> Ray;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    
}

impl Lambertian {
    pub fn new() -> Material {
        Lambertian {} as Material
    }
    
    pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
        loop {
            let point = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - Vec3::ones();
            if point.squared_length() >= 1.0 {
                break point
            }
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, hit: &Hit, t_min: f32, t_max: f32, rng: &mut ThreadRng) -> Ray {
        let target = hit.point + hit.normal + Lambertian::random_in_unit_sphere(rng);
        Ray::new(hit.point, (target - hit.point).make_unit_vector())
    }
}