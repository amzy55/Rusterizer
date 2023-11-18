use glam::{Vec2, Vec3};
use minifb::{Key, Window, WindowOptions};
use std::path::Path;

use rusterizer::*;
pub mod font;
pub use font::Font;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn input_handling(dt: f32, window: &Window, camera: &mut Camera) {
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
    axis *= dt;
    camera.transform.translation += camera.transform.right() * camera.speed * axis.x
        + camera.transform.forward() * camera.speed * axis.y
        + camera.transform.up() * camera.speed * axis.z;
}

fn main() {
    let grey = from_u8_rgb(100, 100, 200);
    let mut buffer: Vec<u32> = vec![grey; WIDTH * HEIGHT];
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
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let texture = Texture::load(Path::new("assets/textures/bee_icon_256.png"));
    let model = load_gltf(Path::new(
        "assets/gltf_models/damaged_helmet/DamagedHelmet.gltf",
    ));
    let window_size = glam::vec2(WIDTH as f32, HEIGHT as f32);

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let mut camera = Camera {
        aspect_ratio,
        transform: Transform::from_translation(glam::vec3(0.0, 0.0, 5.0)),
        frustum_near: 0.5,
        frustum_far: 100.0,
        ..Default::default()
    };

    // has to be mutable because of how it's implemented
    let mut font = Font::default();

    let mut rot = 0.0;

    let now = std::time::Instant::now();
    let mut start_time = now.elapsed().as_secs_f32();
    let mut fps_timer = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let end_time = now.elapsed().as_secs_f32();
        let dt = end_time - start_time;
        fps_timer += dt;
        if fps_timer > 1.0 {
            println!("{}", (fps_timer / dt) as u32);
            fps_timer = 0.0;
        }

        input_handling(dt, &window, &mut camera);
        buffer.fill(grey);
        z_buffer.fill(f32::INFINITY);
        let parent_local =
            Transform::from_rotation(glam::Quat::from_euler(glam::EulerRot::XYZ, rot, 0.0, 0.0))
                .local();
        let mvp = camera.projection() * camera.view() * parent_local;

        raster_mesh(
            &model,
            &(mvp),
            &parent_local,
            Some(&texture),
            &mut buffer,
            &mut z_buffer,
            window_size,
        );

        let _text_mvp = camera.projection() * camera.view() * glam::Mat4::IDENTITY;
        let text_pos = Vec2::new(50.0, HEIGHT as f32 / 2.0);
        font.text("The coolest rasterizer ever!".to_string(), text_pos);
        font.render(&mut buffer, &mut z_buffer, window_size);

        rot += 0.5 * dt as f32;
        start_time = end_time;
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
