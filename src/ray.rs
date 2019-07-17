use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray
    {
        Ray { 
            origin: origin, 
            direction: direction 
            }
    }

    pub fn point_at_time_t(self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

pub trait Hitable {
    fn is_hit_by(&self, ray: &Ray) -> bool;
}