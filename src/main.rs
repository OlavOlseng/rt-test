extern crate minifb;

use minifb::{Key, WindowOptions, Window};

mod vec3;
use crate::vec3::*;

mod ray;
use ray::Ray;

mod hit;
use crate::hit::*;

mod sphere;
use crate::sphere::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;

fn main() {

    let mut screen_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut spheres: Vec<Sphere> = Vec::new(); 
    spheres.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    spheres.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));
    
    let world = World{ world: spheres };
    
    let mut window = Window::new(
        "Raytracing test", 
        WIDTH, 
        HEIGHT, 
        WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e)
        });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        screen_buffer = render(screen_buffer, WIDTH as u32, HEIGHT as u32, &world);
        window.update_with_buffer(&screen_buffer).unwrap();
    };
}

fn render(mut buffer: Vec<u32>, width: u32, height: u32, world: &World) -> Vec<u32>{
    let mut pixel = 0;
    let buffer_size = buffer.len();

    //Camera/Viewport description?
    // let origin: Vec3 = Vec3::origin();
    let origin: Vec3 = Vec3::origin();
    let lower_left: Vec3 = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.0, 0.0);
    
    for y in 0..height {
        for x in 0..width {
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;

            let direction = lower_left + u * horizontal + v * vertical;

            let ray = Ray::new(origin, direction);


            let color = if let Some(hit) = world.trace(&ray, 0.0, std::f32::MAX) {
                shade_sphere(&hit)
            }
            else {
                to_argb_from_vec3(ray_to_color_vec(ray))
            };                
           
            buffer[buffer_size - pixel - 1] = color;
            pixel += 1;
        }
    }
    buffer
}

fn shade_sphere(hit: &Hit) -> u32 {
    to_argb_from_vec3(0.5 * (hit.normal + Vec3::new(1.0, 1.0, 1.0)))
}

fn ray_to_color_vec(ray: Ray) -> Vec3 {
    let unit_direction = ray.point_at_time_t(1.0).make_unit_vector();
    unit_direction
    // let t = 0.9 * (ray.direction.make_unit_vector().y + 1.0);
    // (1.0 - t) * Vec3::ones() + t * Vec3::new(0.5, 0.7, 1.0)
}

fn to_argb(r: u32, g: u32, b: u32, a: u32) -> u32 {
    a << 24 | r << 16 | g << 8 | b
}

fn to_argb_from_vec3(vector: Vec3) -> u32 {
    let r: u32 = (vector.x * 255.0) as u32;
    let g: u32 = (vector.y * 255.0) as u32;
    let b: u32 = (vector.z * 255.0) as u32;
    to_argb(r, g, b, 255)
}

pub struct World{ world: Vec<Sphere>}

impl World {
    pub fn trace(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut closest = t_max;
        let mut first_hit: Option<Hit> = None;

        for obj in self.world.iter() {
           if let Some(hit) = obj.is_hit_by(t_min, t_max, &ray) {
                if hit.t < closest {
                    closest = hit.t;
                    first_hit = Some(hit);
                }
           }
        }
        first_hit
    }
}