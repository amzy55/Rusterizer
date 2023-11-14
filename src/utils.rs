use glam::{Vec2, Vec3};

pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    u32::from_be_bytes([0, r, g, b])
}

// returns the determinant of the parallelogram defined by the two lines with v0 as a common point
// it is also the are of the parallelogram
pub fn edge_function(point: Vec2, v0: Vec2, v1: Vec2) -> f32 {
    let seg_a = v1 - v0;
    let seg_b = point - v0;

    seg_a.x * seg_b.y - seg_a.y * seg_b.x
}

pub fn barycentric_coords(point: Vec2, v0: Vec2, v1: Vec2, v2: Vec2, triangle_area: f32) -> Option<Vec3> {
    // w - weight
    let w0 = edge_function(point, v0, v1);
    let w1 = edge_function(point, v1, v2);
    let w2 = edge_function(point, v2, v0);

    // 1 division instead of 3
    let reverse_area = 1.0 / triangle_area;

    if w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0 {
        // Vec3::new(w0 * reverse_area, w1 * reverse_area, w2 * reverse_area)
        Some(Vec3{x: w0 * reverse_area, y: w1 * reverse_area, z: w2 * reverse_area})
    }
    else {
        None
    }

}