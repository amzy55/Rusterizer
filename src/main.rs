use minifb::{Key, Window, WindowOptions};
use glam::Vec2;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    u32::from_be_bytes([0, r, g, b])
}

fn edge_function(p: Vec2, v0: Vec2, v1: Vec2) -> f32 {
    let seg_a = v1 - v0;
    let seg_b = p - v0;

    seg_a.x * seg_b.y - seg_a.y * seg_b.x
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

    let move_by: f32 = 5.0;
    let mut offset = Vec2::new(0.0, 0.0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        
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

        for i in 0..(WIDTH * HEIGHT) {
            let mut color = from_u8_rgb(0, 0, 0);


            let x: f32 = (i % WIDTH) as f32 + offset.x;
            let y: f32 = (i / WIDTH) as f32 + offset.y;

            let p = Vec2::new(x, y);
            let a = edge_function(p, v0, v2);
            let b = edge_function(p, v2, v1);
            let c = edge_function(p, v1, v0);

            // if a >= 0.0 && b >= 0.0 && c >= 0.0 {
            //     color = from_u8_rgb(0, 255, 0);
            // } else {
            //     color = from_u8_rgb(255, 0, 0);
            // }

            color = from_u8_rgb((a * 255.0) as u8, (b * 255.0) as u8, (c * 255.0) as u8);

            buffer[i] = color;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
