use glam::{Vec2, Vec3Swizzles};

pub mod geometry;
pub mod texture;
pub mod transform;
pub mod utils;
pub use {
    geometry::{Mesh, Vertex},
    texture::Texture,
    transform::{Transform, TransformInitialParams},
    utils::*,
};

pub fn raster_triangle(
    vertices: &[Vertex],
    texture: Option<&Texture>,
    buffer: &mut Vec<u32>,
    z_buffer: &mut Vec<f32>,
    window_size: Vec2,
    offset: Vec2,
) {
    let triangle_area = edge_function(vertices[0].pos.xy(), vertices[1].pos.xy(), vertices[2].pos.xy());
    // iterating over the buffer
    for (i, pixel) in buffer.iter_mut().enumerate() {
        // +0.5 to take the center of the pixel
        let point = Vec2::new(i as f32 % window_size.x + offset.x, i as f32 / window_size.x + offset.y) + 0.5;
        if let Some(bary) =
            barycentric_coords(point, vertices[0].pos.xy(), vertices[1].pos.xy(), vertices[2].pos.xy(), triangle_area)
        {
            let depth = bary.x * vertices[0].pos.z + bary.y * vertices[1].pos.z + bary.z * vertices[2].pos.z;
            if depth < z_buffer[i] {
                z_buffer[i] = depth;
                match texture {
                    Some(texture) => {
                        let tex_coords = bary.x * vertices[0].uv + bary.y * vertices[1].uv + bary.z * vertices[2].uv;
                        let color = texture.rgb_at_uv(tex_coords.x, tex_coords.y);
                        *pixel = color;
                    }
                    None => {
                        let color = bary.x * vertices[0].color + bary.y * vertices[1].color + bary.z * vertices[2].color;
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

// pub fn raster_mesh(
//     mesh: &Mesh,
//     texture: &Texture,
//     buffer: &mut Vec<u32>,
//     z_buffer: &mut Vec<f32>,
//     window_size: Vec2,
// ) {

// }