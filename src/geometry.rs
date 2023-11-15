use glam::{Vec2, Vec3, Vec3Swizzles};
use crate::utils::*;

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub pos: Vec3,
    pub color: Vec3,
    pub uv: Vec2
}

pub struct Triangle {
    pub v0: Vertex,
    pub v1: Vertex,
    pub v2: Vertex,
    pub triangle_area: f32,
}

impl Triangle {
    pub fn new(v0: Vertex, v1: Vertex, v2: Vertex) -> Self {
        Self {
            v0: v0,
            v1: v1,
            v2: v2,
            triangle_area: edge_function(v0.pos.xy(), v1.pos.xy(), v2.pos.xy()),
        }
    }
}