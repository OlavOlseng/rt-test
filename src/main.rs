extern crate minifb;
use minifb::{Key, WindowOptions, Window};

use rand::prelude::*;

mod vec3;
use crate::vec3::*;

mod ray;
use ray::Ray;

mod hit;
use crate::hit::*;

mod sphere;
use crate::sphere::*;

mod camera;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

const RAY_SAMPLES: u32 = 200;

fn main() {

    let mut screen_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut spheres: Vec<Sphere> = Vec::new(); 
    spheres.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    spheres.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));
    
    let world = World{ world: spheres };
    
    let mut window = Window::new(
        "Raytracing test - esc to exit", 
        WIDTH, 
        HEIGHT, 
        WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e)
        });

    screen_buffer = render(screen_buffer, WIDTH as u32, HEIGHT as u32, &world);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&screen_buffer).unwrap();
    };
}

fn render(mut buffer: Vec<u32>, width: u32, height: u32, world: &World) -> Vec<u32>{
    let mut pixel = 0;
    //Camera/Viewport description?
    // let origin: Vec3 = Vec3::origin();
    
    // let rng = rand::thread_rng();
    let mut rng = thread_rng();
    
    let cam = camera::Camera::new(
            Vec3::origin(),
            Vec3::new(-2.0, -1.0, -1.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0)
        );

    let start = std::time::Instant::now();

    for y in 0..height {
        for x in 0..width {
            // let u = x as f32 / width as f32;
            // let v = 1.0 - y as f32 / height as f32;

            // let ray = Ray::new(origin, direction);
            let mut color = Vec3::origin();

            for _i in 0..RAY_SAMPLES {
                let u = (x as f32 + rng.gen::<f32>()) / width as f32;
                let v = 1.0 - (y as f32 + rng.gen::<f32>()) / height as f32;

                let ray = cam.get_ray(u, v);

                color += world.trace(&ray, 0.0001, std::f32::MAX, &mut rng);
            }
            color = color / RAY_SAMPLES as f32;
            // println!("Rasterized pixel : {} {:?}", pixel, color);
            buffer[pixel] = to_argb_from_vec3(color);
            pixel += 1;
        }
    }
    println!("Frame render time: {}", start.elapsed().as_secs());
    buffer
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
    pub fn trace(&self, ray: &Ray, t_min: f32, t_max: f32, rng: &mut ThreadRng) -> Vec3 {
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
        if first_hit.is_some() {
            let reflection = World::get_diffuse_reflection(&first_hit.unwrap(), rng);
            return 0.5 * self.trace(&reflection, t_min, t_max, rng);
        } else {
            World::ray_to_ambient_color_vec(ray)
        }
    }

    pub fn get_diffuse_reflection(hit: &Hit, rng: &mut ThreadRng) -> Ray {
        let target = hit.point + hit.normal + World::random_in_unit_sphere(rng);
        Ray::new(hit.point, (target - hit.point).make_unit_vector())
    }

    fn ray_to_ambient_color_vec(ray: &Ray) -> Vec3 {
        // let unit_direction = ray.point_at_time_t(1.0).make_unit_vector();
        // unit_direction
        let t = 0.5 * (ray.direction.make_unit_vector().y + 1.0);
        (1.0 - t) * Vec3::ones() + t * Vec3::new(0.5, 0.7, 1.0)
    }

    fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
        loop {
            let point = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - Vec3::ones();
            if point.squared_length() >= 1.0 {
                break point
            }
        }
    }
}