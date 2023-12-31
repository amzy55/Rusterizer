use std::path::Path;

use glam::{Mat4, Vec2, Vec3, Vec4Swizzles};

pub mod camera;
pub mod geometry;
pub mod texture;
pub mod transform;
pub mod utils;
pub use {
    camera::Camera,
    geometry::*,
    texture::Texture,
    transform::{Transform, TransformInitialParams},
    utils::*,
};

pub fn raster_clipped_triangle(
    triangle: &Triangle,
    texture: Option<&Texture>,
    buffer: &mut Vec<u32>,
    z_buffer: &mut Vec<f32>,
    viewport_size: Vec2,
) {
    let rec0 = 1.0 / triangle.v0.pos.w;
    let rec1 = 1.0 / triangle.v1.pos.w;
    let rec2 = 1.0 / triangle.v2.pos.w;

    // normalized device coordinates -> between -1 and 1
    let ndc0 = triangle.v0.pos * rec0;
    let ndc1 = triangle.v1.pos * rec1;
    let ndc2 = triangle.v2.pos * rec2;

    // perspective division on all attributes
    let pv0 = triangle.v0 * rec0;
    let pv1 = triangle.v1 * rec1;
    let pv2 = triangle.v2 * rec2;

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
    // bb - bounding box of the triangle
    if let Some(bb) = triangle_screen_bounding_box(&[sc0, sc1, sc2], viewport_size) {
        for y in (bb.top as usize)..=bb.bottom as usize {
            for x in (bb.left as usize)..=bb.right as usize {
                // +0.5 to take the center of the pixel
                let coords = glam::vec2(x as f32, y as f32) + 0.5;
                let pixel_id = coords_to_index(x, y, viewport_size.x as usize);
                if let Some(bary) = barycentric_coords(coords, sc0, sc1, sc2, triangle_area) {
                    let correction = bary.x * rec0 + bary.y * rec1 + bary.z * rec2;
                    let correction = 1.0 / correction;
                    let depth = bary.x * ndc0.z + bary.y * ndc1.z + bary.z * ndc2.z;
                    if depth < z_buffer[pixel_id] {
                        z_buffer[pixel_id] = depth;
                        let normal =
                            bary.x * pv0.normal + bary.y * pv1.normal + bary.z * pv2.normal;
                        let normal = normal * correction;
                        let n_dot_l = normal.dot(Vec3::ONE.normalize()); // light in the position (1, 1, 1)
                        let ambient = glam::vec3(0.3, 0.3, 0.3); // fixed amount of ambient light
                        let color = bary.x * pv0.color + bary.y * pv1.color + bary.z * pv2.color;
                        let color = color * correction * n_dot_l + ambient;
                        match texture {
                            Some(texture) => {
                                let tex_coords =
                                    bary.x * pv0.uv + bary.y * pv1.uv + bary.z * pv2.uv;
                                let tex_coords = tex_coords * correction;
                                let tex_color = texture.rgb_at_uv(tex_coords.x, tex_coords.y);
                                let r = (tex_color >> 16) as u8;
                                let g = (tex_color >> 8) as u8;
                                let b = tex_color as u8;
                                buffer[pixel_id] = from_u8_rgb(
                                    (r as f32 * color.x) as u8,
                                    (g as f32 * color.y) as u8,
                                    (b as f32 * color.z) as u8,
                                );
                            }
                            None => {
                                buffer[pixel_id] = from_u8_rgb(
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
    }
}

pub fn raster_triangle(
    vertices: &[Vertex; 3],
    mvp: &Mat4,
    model_matrix: &Mat4,
    texture: Option<&Texture>,
    buffer: &mut Vec<u32>,
    z_buffer: &mut Vec<f32>,
    viewport_size: Vec2,
) {
    let trans_inv = glam::Mat4::transpose(&glam::Mat4::inverse(model_matrix));

    let triangle = Triangle {
        v0: vertices[0],
        v1: vertices[1],
        v2: vertices[2],
    };
    let mut clip_tri = triangle.transform(mvp);
    clip_tri.v0.normal = (trans_inv * clip_tri.v0.normal.extend(0.0)).xyz();
    clip_tri.v1.normal = (trans_inv * clip_tri.v1.normal.extend(0.0)).xyz();
    clip_tri.v2.normal = (trans_inv * clip_tri.v2.normal.extend(0.0)).xyz();

    match clip_cull_triangle(&clip_tri) {
        ClipResult::None => {}
        ClipResult::One(tri) => {
            raster_clipped_triangle(&tri, texture, buffer, z_buffer, viewport_size);
        }
        ClipResult::Two(tri) => {
            raster_clipped_triangle(&tri.0, texture, buffer, z_buffer, viewport_size);
            raster_clipped_triangle(&tri.1, texture, buffer, z_buffer, viewport_size);
        }
    }
}

pub fn raster_mesh(
    mesh: &Mesh,
    model_matrix: &Mat4,
    mvp: &Mat4,
    texture: Option<&Texture>,
    buffer: &mut Vec<u32>,
    z_buffer: &mut Vec<f32>,
    viewport_size: Vec2,
) {
    for triangle_indices in &mesh.triangle_indices {
        let vertices = mesh.get_vertices_from_triangle_indices(*triangle_indices);
        raster_triangle(
            &vertices,
            model_matrix,
            mvp,
            texture,
            buffer,
            z_buffer,
            viewport_size,
        );
    }
}

pub fn triangle_screen_bounding_box(
    poss: &[Vec2; 3],
    viewport_size: Vec2,
) -> Option<BoundingBox2D> {
    let bb = get_triangle_bounding_box_2d(poss);

    if bb.left >= viewport_size.x || bb.right < 0.0 || bb.bottom >= viewport_size.y || bb.top < 0.0
    {
        None
    } else {
        let left = bb.left.max(0.0);
        let right = bb.right.min(viewport_size.x - 1.0);
        let bottom = bb.bottom.max(0.0);
        let top = bb.top.min(viewport_size.y - 1.0);

        Some(BoundingBox2D {
            left,
            right,
            top,
            bottom,
        })
    }
}

pub enum ClipResult {
    None,
    One(Triangle),
    Two((Triangle, Triangle)),
}

//View Frustum Culling
pub fn cull_triangle_view_frustum(triangle: &Triangle) -> bool {
    // cull tests against the 6 planes
    if triangle.v0.pos.x > triangle.v0.pos.w
        && triangle.v1.pos.x > triangle.v1.pos.w
        && triangle.v2.pos.x > triangle.v2.pos.w
    {
        return true;
    }
    if triangle.v0.pos.x < -triangle.v0.pos.w
        && triangle.v1.pos.x < -triangle.v1.pos.w
        && triangle.v2.pos.x < -triangle.v2.pos.w
    {
        return true;
    }
    if triangle.v0.pos.y > triangle.v0.pos.w
        && triangle.v1.pos.y > triangle.v1.pos.w
        && triangle.v2.pos.y > triangle.v2.pos.w
    {
        return true;
    }
    if triangle.v0.pos.y < -triangle.v0.pos.w
        && triangle.v1.pos.y < -triangle.v1.pos.w
        && triangle.v2.pos.y < -triangle.v2.pos.w
    {
        return true;
    }
    if triangle.v0.pos.z > triangle.v0.pos.w
        && triangle.v1.pos.z > triangle.v1.pos.w
        && triangle.v2.pos.z > triangle.v2.pos.w
    {
        return true;
    }
    if triangle.v0.pos.z < 0.0 && triangle.v1.pos.z < 0.0 && triangle.v2.pos.z < 0.0 {
        return true;
    }

    false
}

pub fn clip_triangle_two(triangle: &Triangle) -> (Triangle, Triangle) {
    // calculate alpha values for getting adjusted vertices
    let alpha_a = (-triangle.v0.pos.z) / (triangle.v1.pos.z - triangle.v0.pos.z);
    let alpha_b = (-triangle.v0.pos.z) / (triangle.v2.pos.z - triangle.v0.pos.z);

    // interpolate to get v0a and v0b
    let v0_a = lerp(triangle.v0, triangle.v1, alpha_a);
    let v0_b = lerp(triangle.v0, triangle.v2, alpha_b);

    // draw triangles
    let mut result_a = *triangle;
    let mut result_b = *triangle;

    result_a.v0 = v0_a;

    result_b.v0 = v0_a;
    result_b.v1 = v0_b;

    let green = Vec3::new(0.0, 1.0, 0.0);
    let blue = Vec3::new(0.0, 0.0, 1.0);

    result_a.v0.color = green;
    result_a.v1.color = green;
    result_a.v2.color = green;
    result_b.v0.color = blue;
    result_b.v1.color = blue;
    result_b.v2.color = blue;

    (result_a, result_b)
}

pub fn clip_triangle_one(triangle: &Triangle) -> Triangle {
    // calculate alpha values for getting adjusted vertices
    let alpha_a = (-triangle.v0.pos.z) / (triangle.v2.pos.z - triangle.v0.pos.z);
    let alpha_b = (-triangle.v1.pos.z) / (triangle.v2.pos.z - triangle.v1.pos.z);

    // interpolate to get v0a and v0b
    let mut v0 = lerp(triangle.v0, triangle.v2, alpha_a);
    let mut v1 = lerp(triangle.v1, triangle.v2, alpha_b);

    let mut v2 = triangle.v2;

    let red = Vec3::new(1.0, 0.0, 0.0);

    v0.color = red;
    v1.color = red;
    v2.color = red;

    // draw triangles
    Triangle { v0, v1, v2 }
}

pub fn clip_cull_triangle(triangle: &Triangle) -> ClipResult {
    /*if cull_triangle_backface(triangle) {
        ClipResult::None
    } else*/
    if cull_triangle_view_frustum(triangle) {
        ClipResult::None
    } else {
        // clipping routines
        if triangle.v0.pos.z < 0.0 {
            if triangle.v1.pos.z < 0.0 {
                ClipResult::One(clip_triangle_one(triangle))
            } else if triangle.v2.pos.z < 0.0 {
                ClipResult::One(clip_triangle_one(&triangle.reorder(VerticesOrder::ACB)))
            } else {
                ClipResult::Two(clip_triangle_two(&triangle.reorder(VerticesOrder::ACB)))
            }
        } else if triangle.v1.pos.z < 0.0 {
            if triangle.v2.pos.z < 0.0 {
                ClipResult::One(clip_triangle_one(&triangle.reorder(VerticesOrder::BCA)))
            } else {
                ClipResult::Two(clip_triangle_two(&triangle.reorder(VerticesOrder::BAC)))
            }
        } else if triangle.v2.pos.z < 0.0 {
            ClipResult::Two(clip_triangle_two(&triangle.reorder(VerticesOrder::CBA)))
        } else {
            // no near clipping necessary
            //return original
            ClipResult::One(*triangle)
        }
    }
}

// don't fully understand this function
// but I know it's checking to see if the triangle is (somewhat) facing the camera
pub fn cull_triangle_backface(triangle: &Triangle) -> bool {
    let normal = (triangle.v1.pos.xyz() - triangle.v1.pos.w)
        .cross(triangle.v2.pos.xyz() - triangle.v0.pos.xyz()); // should be divided by w?
                                                               // any is vertex valid
    let view_dir = -Vec3::Z;
    // also we don't care about normalizing
    // if negative facing the camera
    normal.dot(view_dir) >= 0.0
}

pub fn load_gltf(path: &Path) -> Mesh {
    // handle loading textures, cameras, meshes here
    let (document, buffers, _images) = gltf::import(path).unwrap();

    for scene in document.scenes() {
        for node in scene.nodes() {
            println!(
                "Node #{} has {} children, camera: {:?}, mesh: {:?}, transform: {:?}",
                node.index(),
                node.children().count(),
                node.camera(),
                node.mesh().is_some(),
                node.transform(),
            );
            println!(
                "Node #{} has transform: trans {:?}, rot {:?}, scale {:?},",
                node.index(),
                node.transform().decomposed().0,
                node.transform().decomposed().1,
                node.transform().decomposed().2,
            );
            if let Some(mesh) = node.mesh() {
                return Mesh::load_from_gltf(&mesh, &buffers);
            }
        }
    }

    Mesh::new()
}

pub fn raster_triangle_2d(
    triangle: &Triangle,
    texture: Option<&Texture>,
    buffer: &mut Vec<u32>,
    z_buffer: &mut Vec<f32>,
    viewport_size: Vec2,
) {
    let v0 = triangle.v0;
    let v1 = triangle.v1;
    let v2 = triangle.v2;

    let triangle_area = edge_function(v1.pos.xy(), v2.pos.xy(), v0.pos.xy());
    // bb - bounding box of the triangle
    if let Some(bb) =
        triangle_screen_bounding_box(&[v0.pos.xy(), v1.pos.xy(), v2.pos.xy()], viewport_size)
    {
        for y in (bb.top as usize)..=bb.bottom as usize {
            for x in (bb.left as usize)..=bb.right as usize {
                // +0.5 to take the center of the pixel
                let coords = glam::vec2(x as f32, y as f32) + 0.5;
                let pixel_id = coords_to_index(x, y, viewport_size.x as usize);
                if let Some(bary) =
                    barycentric_coords(coords, v0.pos.xy(), v1.pos.xy(), v2.pos.xy(), triangle_area)
                {
                    let color = bary.x * v0.color + bary.y * v1.color + bary.z * v2.color;
                    let depth = bary.x * v0.pos.z + bary.y * v1.pos.z + bary.z * v2.pos.z;
                    if depth <= z_buffer[pixel_id] {
                        z_buffer[pixel_id] = depth;
                        match &texture {
                            Some(texture) => {
                                let tex_coords = bary.x * v0.uv + bary.y * v1.uv + bary.z * v2.uv;
                                let tex_color = texture.rgb_at_uv(tex_coords.x, tex_coords.y);
                                let alpha = (tex_color >> 24) as u8;
                                let red = (tex_color >> 16) as u8;
                                let green = (tex_color >> 8) as u8;
                                let blue = tex_color as u8;
                                if alpha != 0 {
                                    buffer[pixel_id] = from_u8_argb(
                                        alpha,
                                        (red as f32 * color.x) as u8,
                                        (green as f32 * color.y) as u8,
                                        (blue as f32 * color.z) as u8,
                                    );
                                }
                            }
                            None => {
                                buffer[pixel_id] = from_u8_rgb(
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
    }
}

pub fn raster_mesh_2d(
    mesh: &Mesh,
    texture: Option<&Texture>,
    buffer: &mut Vec<u32>,
    z_buffer: &mut Vec<f32>,
    viewport_size: Vec2,
) {
    for triangle_indices in &mesh.triangle_indices {
        let vertices = mesh.get_vertices_from_triangle_indices(*triangle_indices);
        let triangle = Triangle {
            v0: vertices[0],
            v1: vertices[1],
            v2: vertices[2],
        };
        raster_triangle_2d(&triangle, texture, buffer, z_buffer, viewport_size);
    }
}
