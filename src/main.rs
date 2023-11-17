use glam::{UVec3, Vec2, Vec3, Vec4};
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
    let grey = from_u8_rgb(100, 100, 100);
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

    let side: f32 = 2.0;
    let top_left = Vec2::new(-1.0, -1.0);
    let bottom_right = top_left + side;

    let v0 = Vertex {
        pos: Vec4::new(top_left.x, top_left.y, 1.0, 1.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        color: Vec3::new(1.0, 1.0, 1.0),
        uv: glam::vec2(0.0, 0.0),
    };
    let v1 = Vertex {
        pos: Vec4::new(top_left.x, bottom_right.y, 1.0, 1.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        color: Vec3::new(1.0, 1.0, 1.0),
        uv: glam::vec2(0.0, 1.0),
    };
    let v2 = Vertex {
        pos: Vec4::new(bottom_right.x, top_left.y, 1.0, 1.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        color: Vec3::new(1.0, 1.0, 1.0),
        uv: glam::vec2(1.0, 0.0),
    };
    let v3 = Vertex {
        pos: Vec4::new(bottom_right.x, bottom_right.y, 1.0, 1.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        color: Vec3::new(1.0, 1.0, 1.0),
        uv: glam::vec2(1.0, 1.0),
    };

    let _quad = Mesh {
        triangle_indices: vec![UVec3::new(0, 1, 2), UVec3::new(2, 1, 3)],
        vertices: vec![v0, v1, v2, v3],
    };

    let _transforms = [
        Transform::IDENTITY,
        //-z
        Transform::from_rotation(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            -std::f32::consts::PI,
            0.0,
            0.0,
        )),
        //+y
        Transform::from_rotation(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            std::f32::consts::FRAC_PI_2,
            0.0,
            0.0,
        )),
        //-y
        Transform::from_rotation(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_2,
            0.0,
            0.0,
        )),
        //+x
        Transform::from_rotation(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            0.0,
            -std::f32::consts::FRAC_PI_2,
            0.0,
        )),
        //-x
        Transform::from_rotation(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            0.0,
            std::f32::consts::FRAC_PI_2,
            0.0,
        )),
    ];

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
        // for transform in transforms {
        //     raster_mesh(
        //         &quad,
        //         &(mvp * transform.local()),
        //         &texture,
        //         &mut buffer,
        //         &mut z_buffer,
        //         window_size,
        //     );
        // }

        // raster_mesh(
        //     &model,
        //     &(mvp),
        //     &parent_local,
        //     Some(&texture),
        //     &mut buffer,
        //     &mut z_buffer,
        //     window_size,
        // );

        let _text_mvp = camera.projection() * camera.view() * glam::Mat4::IDENTITY;
        font.text("text!./ ".to_string(), Vec2{ x: -0.5, y: -0.5});
        font.text("*i can type stuff*".to_string(), Vec2{ x: -0.5, y: -0.25});
        font.render(&mut buffer, &mut z_buffer, &mvp, window_size);

        rot += 0.5 * dt as f32;
        start_time = end_time;
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
