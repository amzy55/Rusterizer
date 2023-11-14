use glam::{Vec2, Vec3, Vec3Swizzles};
use minifb::{Key, Window, WindowOptions};
pub mod utils;
pub use utils::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

pub struct Vertex {
    pub pos: Vec3,
    pub color: Vec3,
}

fn raster_triangle(
    v0: &Vertex,
    v1: &Vertex,
    v2: &Vertex,
    triangle_area: f32,
    buffer: &mut Vec<u32>,
    z_buffer: &mut Vec<f32>,
    offset: Vec2,
) {
    // iterating over the buffer
    for (i, pixel) in buffer.iter_mut().enumerate() {
        let point = Vec2::new((i % WIDTH) as f32 - offset.x, (i / WIDTH) as f32 - offset.y);
        if let Some(bary) =
            barycentric_coords(point, v0.pos.xy(), v1.pos.xy(), v2.pos.xy(), triangle_area)
        {
            let depth = bary.x * v0.pos.z + bary.y * v1.pos.z + bary.z * v2.pos.z;
            if depth < z_buffer[i] {
                z_buffer[i] = depth;
                let color = bary.x * v0.color + bary.y * v1.color + bary.z * v2.color;
                *pixel = from_u8_rgb(
                    (color.x * 255.0) as u8,
                    (color.y * 255.0) as u8,
                    (color.z * 255.0) as u8,
                )
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

    let v0 = Vertex {
        pos: Vec3::new(WIDTH as f32 / 3.0, 50.0, 0.0),
        color: Vec3::new(1.0, 1.0, 0.0),
    };
    let v1 = Vertex {
        pos: Vec3::new(WIDTH as f32 / 2.0, 300.0, 0.0),
        color: Vec3::new(1.0, 0.0, 1.0),
    };
    let v2 = Vertex {
        pos: Vec3::new(WIDTH as f32 / 3.0 * 2.0, 50.0, 0.0),
        color: Vec3::new(0.0, 1.0, 1.0),
    };
    let t1_area = edge_function(v0.pos.xy(), v1.pos.xy(), v2.pos.xy());

    let v3 = Vertex {
        pos: Vec3::new(WIDTH as f32 / 3.0 + 50.0, 30.0, 1.0),
        color: Vec3::new(1.0, 0.0, 0.0),
    };
    let v4 = Vertex {
        pos: Vec3::new(WIDTH as f32 / 2.0 + 50.0, 280.0, 1.0),
        color: Vec3::new(0.0, 1.0, 0.0),
    };
    let v5 = Vertex {
        pos: Vec3::new(WIDTH as f32 / 3.0 * 2.0 + 50.0, 30.0, 1.0),
        color: Vec3::new(0.0, 0.0, 1.0),
    };
    let t2_area = edge_function(v0.pos.xy(), v1.pos.xy(), v2.pos.xy());

    let mut offset = Vec2::new(0.0, 0.0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        input_handling(&window, &mut offset);
        buffer.fill(0);
        z_buffer.fill(f32::INFINITY);
        raster_triangle(&v0, &v1, &v2, t1_area, &mut buffer, &mut z_buffer, offset);
        raster_triangle(&v3, &v4, &v5, t2_area, &mut buffer, &mut z_buffer, offset);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
