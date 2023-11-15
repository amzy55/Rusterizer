use glam::{Vec2, Vec3, UVec3};

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
}

impl Triangle {
    pub fn new(v0: Vertex, v1: Vertex, v2: Vertex) -> Self {
        Self {
            v0: v0,
            v1: v1,
            v2: v2,
        }
    }
}

pub struct Mesh {
    pub triangle_indices: Vec<UVec3>,
    pub vertices: Vec<Vertex>
}

impl Mesh {
    pub fn get_vertices_from_triangle_indices(&self, triangle_indices: UVec3) -> [&Vertex; 3] {
        [
            &self.vertices[triangle_indices.x as usize],
            &self.vertices[triangle_indices.y as usize],
            &self.vertices[triangle_indices.z as usize],
        ]
    }
}