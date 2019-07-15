extern crate minifb;

use minifb::{Key, WindowOptions, Window};

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


    screen_buffer = render(screen_buffer, WIDTH as f32, HEIGHT as f32);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        
       window.update_with_buffer(&screen_buffer).unwrap();
    }
  println!("Hello, world!");
}

fn render(mut buffer: Vec<u32>, width: f32, height: f32) -> Vec<u32>{
    let mut pixel = 0;
    let mut y = 0.0;
    while y < height {
        let mut x = 0.0;        
        while x < width {
            let r: u32 = (x / width * 256.0) as u32;
            let g: u32 = (y / height * 256.0) as u32;
            let b: u32 = 128;
            buffer[pixel] = 255 << 24 | r << 16 | g << 8 | b;
            x += 1.0;
            pixel += 1;
        }
        y += 1.0;
    }
    buffer
}

