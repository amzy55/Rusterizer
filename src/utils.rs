use glam::Vec2;

pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    u32::from_be_bytes([0, r, g, b])
}

pub fn edge_function(p: Vec2, v0: Vec2, v1: Vec2) -> f32 {
    let seg_a = v1 - v0;
    let seg_b = p - v0;

    seg_a.x * seg_b.y - seg_a.y * seg_b.x
}