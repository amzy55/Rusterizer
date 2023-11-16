use glam::{UVec3, Vec2, Vec3};
use minifb::{Key, Window, WindowOptions};
use std::path::Path;

use rusterizer::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn input_handling(window: &Window, camera: &mut Camera) {
    let mut axis = Vec3::new(0.0, 0.0, 0.0);
    if window.is_key_down(Key::A) {
        axis.x -= 1.0;
    }
    if window.is_key_down(Key::D) {
        axis.x += 1.0;
    }
    if window.is_key_down(Key::W) {
        axis.y += 1.0;
    }
    if window.is_key_down(Key::S) {
        axis.y -= 1.0;
    }
    if window.is_key_down(Key::Q) {
        axis.z -= 1.0;
    }
    if window.is_key_down(Key::E) {
        axis.z += 1.0;
    }
    camera.transform.translation += camera.transform.right() * camera.speed * axis.x
        + camera.transform.forward() * camera.speed * axis.y
        + camera.transform.up() * camera.speed * axis.z;
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut z_buffer = vec![f32::INFINITY; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Rusterizer - ESC to exit",
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

    let side: f32 = 1.0;
    let top_left = Vec2::new(-0.5, -0.5);
    let bottom_right = top_left + side;

    let v0 = Vertex {
        pos: Vec3::new(top_left.x, top_left.y, 0.0),
        color: Vec3::new(1.0, 1.0, 0.0),
        uv: glam::vec2(0.0, 0.0),
    };
    let v1 = Vertex {
        pos: Vec3::new(top_left.x, bottom_right.y, 0.0),
        color: Vec3::new(1.0, 0.0, 1.0),
        uv: glam::vec2(0.0, 1.0),
    };
    let v2 = Vertex {
        pos: Vec3::new(bottom_right.x, top_left.y, 0.0),
        color: Vec3::new(0.0, 1.0, 1.0),
        uv: glam::vec2(1.0, 0.0),
    };
    let v3 = Vertex {
        pos: Vec3::new(bottom_right.x, bottom_right.y, 0.0),
        color: Vec3::new(0.0, 1.0, 1.0),
        uv: glam::vec2(1.0, 1.0),
    };

    let quad = Mesh {
        triangle_indices: vec![UVec3::new(0, 1, 2), UVec3::new(2, 1, 3)],
        vertices: vec![v0, v1, v2, v3],
    };

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let mut camera = Camera {
        aspect_ratio,
        transform: Transform::from_translation(glam::vec3(0.0, 0.0, 1.5)),
        frustum_far: 100.0,
        ..Default::default()
    };

    let mut rot = 0.0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        input_handling(&window, &mut camera);
        buffer.fill(0);
        z_buffer.fill(f32::INFINITY);
        let mesh_transform =
            Transform::from_rotation(glam::Quat::from_euler(glam::EulerRot::XYZ, rot, 0.0, 0.0));
        let mvp = camera.projection() * camera.view() * mesh_transform.local();
        raster_mesh(
            &quad,
            &mvp,
            &texture,
            &mut buffer,
            &mut z_buffer,
            window_size,
        );
        rot += 0.05;
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
