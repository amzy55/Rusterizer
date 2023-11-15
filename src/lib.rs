use glam::{Mat4, Vec2, Vec4};

pub mod camera;
pub mod geometry;
pub mod texture;
pub mod transform;
pub mod utils;
pub use {
    camera::Camera,
    geometry::{Mesh, Vertex},
    texture::Texture,
    transform::{Transform, TransformInitialParams},
    utils::*,
};

pub fn raster_triangle(
    vertices: &[Vertex; 3],
    model: &Mat4,
    view: &Mat4,
    projection: &Mat4,
    texture: Option<&Texture>,
    buffer: &mut Vec<u32>,
    z_buffer: &mut Vec<f32>,
    viewport_size: Vec2,
) {
    let mvp = *projection * *view * *model;

    let clip0 = mvp * Vec4::from((vertices[0].pos, 1.0));
    let clip1 = mvp * Vec4::from((vertices[1].pos, 1.0));
    let clip2 = mvp * Vec4::from((vertices[2].pos, 1.0));

    let rec0 = 1.0 / clip0.w;
    let rec1 = 1.0 / clip1.w;
    let rec2 = 1.0 / clip2.w;

    let uv0 = vertices[0].uv * rec0;
    let uv1 = vertices[1].uv * rec1;
    let uv2 = vertices[2].uv * rec2;

    let color0 = vertices[0].color * rec0;
    let color1 = vertices[1].color * rec1;
    let color2 = vertices[2].color * rec2;

    // normalized device coordinates -> between -1 and 1
    let ndc0 = clip0 * rec0;
    let ndc1 = clip1 * rec1;
    let ndc2 = clip2 * rec2;

    // screen coordinates remapped to window
    let sc0 = glam::vec2(
        map_to_range(ndc0.x, -1.0, 1.0, 0.0, viewport_size.x),
        map_to_range(ndc0.y, -1.0, 1.0, 0.0, viewport_size.y),
    );
    let sc1 = glam::vec2(
        map_to_range(ndc1.x, -1.0, 1.0, 0.0, viewport_size.x),
        map_to_range(ndc1.y, -1.0, 1.0, 0.0, viewport_size.y),
    );
    let sc2 = glam::vec2(
        map_to_range(ndc2.x, -1.0, 1.0, 0.0, viewport_size.x),
        map_to_range(ndc2.y, -1.0, 1.0, 0.0, viewport_size.y),
    );

    let triangle_area = edge_function(sc0, sc1, sc2);
    // iterating over the buffer
    for (i, pixel) in buffer.iter_mut().enumerate() {
        // +0.5 to take the center of the pixel
        let point = Vec2::new(
            (i as f32 + 0.5) % viewport_size.x,
            (i as f32 + 0.5) / viewport_size.x,
        );
        if let Some(bary) = barycentric_coords(point, sc0, sc1, sc2, triangle_area) {
            let correction = bary.x * rec0 + bary.y * rec1 + bary.z * rec2;
            let correction = 1.0 / correction;
            let depth = bary.x * vertices[0].pos.z
                + bary.y * vertices[1].pos.z
                + bary.z * vertices[2].pos.z;
            if depth < z_buffer[i] {
                z_buffer[i] = depth;
                let color = bary.x * vertices[0].color
                    + bary.y * vertices[1].color
                    + bary.z * vertices[2].color;
                let color = color * correction;
                match texture {
                    Some(texture) => {
                        let tex_coords = bary.x * uv0
                            + bary.y * uv1
                            + bary.z * uv2;
                        let tex_coords = tex_coords * correction;
                        let tex_color = texture.rgb_at_uv(tex_coords.x, tex_coords.y);
                        let r = (tex_color >> 16) as u8;
                        let g = (tex_color >> 8) as u8;
                        let b = tex_color as u8;
                        *pixel = from_u8_rgb(
                            (r as f32 * color.x) as u8,
                            (g as f32 * color.y) as u8,
                            (b as f32 * color.z) as u8,
                        );
                    }
                    None => {
                        *pixel = from_u8_rgb(
                            (color.x * 255.0) as u8,
                            (color.y * 255.0) as u8,
                            (color.z * 255.0) as u8,
                        );
                    }
                }
            }
        }
    }
}

pub fn raster_mesh(
    mesh: &Mesh,
    model: &Mat4,
    view: &Mat4,
    projection: &Mat4,
    texture: &Texture,
    buffer: &mut Vec<u32>,
    z_buffer: &mut Vec<f32>,
    viewport_size: Vec2,
) {
    for triangle_indices in &mesh.triangle_indices {
        let vertices = mesh.get_vertices_from_triangle_indices(*triangle_indices);
        raster_triangle(
            &vertices,
            model,
            view,
            projection,
            Some(texture),
            buffer,
            z_buffer,
            viewport_size,
        );
    }
}
