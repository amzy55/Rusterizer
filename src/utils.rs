use glam::{Vec2, Vec3};

pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    u32::from_be_bytes([0, r, g, b])
}

pub fn index_to_coords(p: usize, width: usize) -> (usize, usize) {
    (p % width, p / width)
}

pub fn coords_to_index(x: usize, y: usize, width: usize) -> usize {
    x + y * width
}

// returns the determinant of the parallelogram defined by the two lines with v0 as a common point
// it is also the are of the parallelogram
pub fn edge_function(point: Vec2, v0: Vec2, v1: Vec2) -> f32 {
    let seg_a = v1 - v0;
    let seg_b = point - v0;

    seg_a.x * seg_b.y - seg_a.y * seg_b.x
}

pub fn barycentric_coords(
    point: Vec2,
    v0: Vec2,
    v1: Vec2,
    v2: Vec2,
    triangle_area: f32,
) -> Option<Vec3> {
    // 1 division instead of 3
    let reverse_area = 1.0 / triangle_area;

    // w - weight
    let w0 = edge_function(point, v1, v2) * reverse_area;
    let w1 = edge_function(point, v2, v0) * reverse_area;
    let w2 = 1.0 - w0 - w1; // no need t do a third edge function because w0 + w1 + w2 = 1.0

    if (w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0) || (w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0) {
        Some(Vec3 {
            x: w0,
            y: w1,
            z: w2,
        })
    } else {
        None
    }
}

pub fn map_to_range<T>(v: T, a1: T, a2: T, b1: T, b2: T) -> T
where
    T: std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Copy,
{
    b1 + (v - a1) * (b2 - b1) / (a2 - a1)
}

pub fn lerp<T>(start: T, end: T, alpha: f32) -> T
where
    T: std::ops::Sub<Output = T>
        + std::ops::Mul<f32, Output = T>
        + std::ops::Add<Output = T>
        + Copy,
{
    start + (end - start) * alpha
}
