use glam::{Vec2, Vec3, Vec3Swizzles};
use minifb::{Key, Window, WindowOptions};
use std::path::Path;
pub mod utils;
pub use utils::*;
pub mod geometry;
pub use geometry::*;
pub mod texture;
pub use texture::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn raster_triangle(
    triangle: &Triangle,
    buffer: &mut Vec<u32>,
    z_buffer: &mut Vec<f32>,
    offset: Vec2,
) {
    // iterating over the buffer
    for (i, pixel) in buffer.iter_mut().enumerate() {
        // +0.5 to take the center of the pixel
        let point = Vec2::new((i % WIDTH) as f32 - offset.x, (i / WIDTH) as f32 - offset.y) + 0.5;
        if let Some(bary) =
            barycentric_coords(point, triangle.v0.pos.xy(), triangle.v1.pos.xy(), triangle.v2.pos.xy(), triangle.triangle_area)
        {
            let depth = bary.x * triangle.v0.pos.z + bary.y * triangle.v1.pos.z + bary.z * triangle.v2.pos.z;
            if depth < z_buffer[i] {
                z_buffer[i] = depth;
                match &triangle.texture {
                    Some(texture) => {
                        let tex_coords = bary.x * triangle.v0.uv + bary.y * triangle.v1.uv + bary.z * triangle.v2.uv;
                        let color = texture.rgb_at_uv(tex_coords.x, tex_coords.y);
                        *pixel = color;
                    }
                    None => {
                        let color = bary.x * triangle.v0.color + bary.y * triangle.v1.color + bary.z * triangle.v2.color;
                        *pixel = from_u8_rgb(
                        (color.x * 255.0) as u8,
                        (color.y * 255.0) as u8,
                        (color.z * 255.0) as u8,
                        )
                    }
                }
            }
        }
    }
}

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
    let texture_copy = Texture::load(Path::new("assets/giorno_stare_1024.jpg"));

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
        Some(&texture)
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
        Some(&texture)
    );

    let mut offset = Vec2::new(0.0, 0.0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        input_handling(&window, &mut offset);
        buffer.fill(0);
        z_buffer.fill(f32::INFINITY);
        raster_triangle(&triangle1, &mut buffer, &mut z_buffer, offset);
        raster_triangle(&triangle2, &mut buffer, &mut z_buffer, offset);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
