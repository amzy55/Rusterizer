use glam::{Vec2, Vec3};
use minifb::{Key, Window, WindowOptions};
use std::path::Path;

use rusterizer::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn input_handling(window: &Window, offset: &mut Vec2) {
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
    let mut z_buffer = vec![f32::INFINITY; WIDTH * HEIGHT];

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

    let texture = Texture::load(Path::new("assets/giorno_stare_1024.jpg"));
    let window_size = glam::vec2(WIDTH as f32, HEIGHT as f32);

    let side: f32 = 300.0;
    let top_left = Vec2::new(200.0, 30.0);
    let bottom_right = top_left + side;

    let triangle1 = Triangle::new(
        Vertex {
            pos: Vec3::new(top_left.x, top_left.y, 0.0),
            color: Vec3::new(1.0, 1.0, 0.0),
            uv: glam::vec2(0.0, 0.0),
        },
        Vertex {
            pos: Vec3::new(top_left.x, bottom_right.y, 0.0),
            color: Vec3::new(1.0, 0.0, 1.0),
            uv: glam::vec2(0.0, 1.0),
        },
        Vertex {
            pos: Vec3::new(bottom_right.x, top_left.y, 0.0),
            color: Vec3::new(0.0, 1.0, 1.0),
            uv: glam::vec2(1.0, 0.0),
        },
    );

    let triangle2 = Triangle::new(
        Vertex {
            pos: Vec3::new(bottom_right.x, top_left.y, 0.0),
            color: Vec3::new(1.0, 1.0, 0.0),
            uv: glam::vec2(1.0, 0.0),
        },
        Vertex {
            pos: Vec3::new(top_left.x, bottom_right.y, 0.0),
            color: Vec3::new(1.0, 0.0, 1.0),
            uv: glam::vec2(0.0, 1.0),
        },
        Vertex {
            pos: Vec3::new(bottom_right.x, bottom_right.y, 0.0),
            color: Vec3::new(0.0, 1.0, 1.0),
            uv: glam::vec2(1.0, 1.0),
        },
    );

    let mut offset = Vec2::new(0.0, 0.0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        input_handling(&window, &mut offset);
        buffer.fill(0);
        z_buffer.fill(f32::INFINITY);
        raster_triangle(&triangle1, Some(&texture), &mut buffer, &mut z_buffer, window_size,  offset);
        raster_triangle(&triangle2, Some(&texture),&mut buffer, &mut z_buffer, window_size, offset);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
