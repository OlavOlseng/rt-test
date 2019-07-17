use crate::vec3::*;
use crate::ray::*;

pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32
}

impl Hit {
    pub fn new(point: Vec3, normal: Vec3, t: f32) -> Hit{
        Hit {
            point: point,
            normal: normal,
            t: t
        }
    }
}

pub trait Hitable {
    fn is_hit_by(&self, t_min: f32, t_max: f32, ray: &Ray) -> Option<Hit>;
}