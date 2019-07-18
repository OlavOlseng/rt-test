use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    vertical: Vec3,
    horizontal: Vec3
}

impl Camera {
    
    pub fn new(origin: Vec3, lower_left: Vec3, horizontal: Vec3,vertical: Vec3) -> Camera {
        Camera {
            origin,
            lower_left,
            vertical,
            horizontal
        }
    }
    
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left + u * self.horizontal + v * self.vertical - self.origin)
    }
}