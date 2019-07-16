extern crate minifb;

use minifb::{Key, WindowOptions, Window};

mod vec3;

use vec3::Vec3;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {

    let mut screen_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    
    let mut window = Window::new(
        "Raytracing test", 
        WIDTH, 
        HEIGHT, 
        WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e)
        });


    screen_buffer = render(screen_buffer, WIDTH as u32, HEIGHT as u32);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        
       window.update_with_buffer(&screen_buffer).unwrap();
    }

    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(3.0, 5.0, 7.0);

    let c = a + b;

    println!("Vec sum: {:?}", c)

}

fn render(mut buffer: Vec<u32>, width: u32, height: u32) -> Vec<u32>{
    let mut pixel = 0;
    for y in 0..height {
        for x in 0..width {
            let r: u32 = (x as f32 / width as f32 * 256.0) as u32;
            let g: u32 = (y as f32 / height as f32 * 256.0) as u32;
            let b: u32 = (pixel % 256) as u32;
            buffer[pixel] = to_argb(r, g, b, 255);
            pixel += 1;
        }
    }
    buffer
}

fn to_argb(r: u32, g: u32, b: u32, a: u32) -> u32 {
    a << 24 | r << 16 | g << 8 | b
}
