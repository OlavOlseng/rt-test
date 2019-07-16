extern crate minifb;

use minifb::{Key, WindowOptions, Window};

mod vec3;
use crate::vec3::*;

mod ray;
use ray::Ray;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;

fn main() {

    let mut screen_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    
    let mut window = Window::new(
        "Raytracing test", 
        WIDTH, 
        HEIGHT, 
        WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e)
        });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        screen_buffer = render(screen_buffer, WIDTH as u32, HEIGHT as u32);
        window.update_with_buffer(&screen_buffer).unwrap();
    };
}

fn render(mut buffer: Vec<u32>, width: u32, height: u32) -> Vec<u32>{
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

            buffer[buffer_size - pixel - 1] = to_argb_from_vec3(ray_to_color_vec(ray));
            pixel += 1;
        }
    }
    buffer
}

fn ray_to_color_vec(ray: Ray) -> Vec3 {
    // ray.direction.make_unit_vector()
    let unit_direction = ray.point_at_time_t(1.0).make_unit_vector();
    // let t = 0.9 * (unit_direction.y + 1.0);
    // (1.0 - t) * Vec3::ones() + t * Vec3::new(0.5, 0.7, 1.0)
    unit_direction
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
