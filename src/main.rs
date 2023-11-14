use minifb::{Key, Window, WindowOptions};
use glam::Vec2;
pub mod utils;
pub use utils::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn input_handling(window: &Window,  offset: &mut Vec2) {
    let move_by: f32 = 5.0;
    if window.is_key_down(Key::A) {
        offset.x -= move_by;
     }
     if window.is_key_down(Key::D) {
        offset.x += move_by;
     }
     if window.is_key_down(Key::W) {
        offset.y -= move_by;
     }
     if window.is_key_down(Key::S) {
        offset.y += move_by;
     }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let v0: Vec2 = Vec2::new(WIDTH as f32 / 3.0, 50.0);
    let v1 = Vec2::new(WIDTH as f32 / 2.0, 300.0);
    let v2 = Vec2::new(WIDTH as f32 / 3.0 * 2.0, 50.0);

    let mut offset = Vec2::new(0.0, 0.0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        input_handling(& window, &mut offset);
        for i in 0..(WIDTH * HEIGHT) {
            let point = Vec2::new(
                (i % WIDTH) as f32 + offset.x,
                (i / WIDTH) as f32 + offset.y);

            let a = edge_function(point, v0, v2);
            let b = edge_function(point, v2, v1);
            let c = edge_function(point, v1, v0);
            
            buffer[i] = from_u8_rgb((a * 255.0) as u8, (b * 255.0) as u8, (c * 255.0) as u8);
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
