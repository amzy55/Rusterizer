use glam::{Mat4, UVec3, Vec2, Vec3, Vec4, Vec4Swizzles};
use std::ops::{Add, Mul, MulAssign, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub pos: Vec4,
    pub color: Vec3,
    pub uv: Vec2,
}

impl Vertex {
    pub fn new(pos: Vec4, color: Vec3, uv: Vec2) -> Self {
        Self { pos, color, uv }
    }
}

impl Add for Vertex {
    type Output = Self;

    fn add(self, right: Self) -> Self {
        let pos = self.pos + right.pos;
        let color = self.color + right.color;
        let uv = self.uv + right.uv;

        Self::new(pos, color, uv)
    }
}

impl Sub for Vertex {
    type Output = Self;

    fn sub(self, right: Self) -> Self {
        let pos = self.pos - right.pos;
        let color = self.color - right.color;
        let uv = self.uv - right.uv;

        Self::new(pos, color, uv)
    }
}

impl Mul for Vertex {
    type Output = Self;

    fn mul(self, right: Self) -> Self {
        let pos = self.pos * right.pos;
        let color = self.color * right.color;
        let uv = self.uv * right.uv;

        Self::new(pos, color, uv)
    }
}

impl Mul<f32> for Vertex {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let pos = self.pos * rhs;
        let color = self.color * rhs;
        let uv = self.uv * rhs;
        Self::new(pos, color, uv)
    }
}

impl MulAssign<f32> for Vertex {
    fn mul_assign(&mut self, rhs: f32) {
        self.pos *= rhs;
        self.color *= rhs;
        self.uv *= rhs;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub v0: Vertex,
    pub v1: Vertex,
    pub v2: Vertex,
}

pub enum VerticesOrder {
    ABC,
    ACB,
    BAC,
    BCA,
    CAB,
    CBA,
}

impl Triangle {
    pub fn new(v0: Vertex, v1: Vertex, v2: Vertex) -> Self {
        Self { v0, v1, v2 }
    }

    pub fn transform(&self, matrix: &Mat4) -> Self {
        let p0 = *matrix * self.v0.pos.xyz().extend(1.0);
        let p1 = *matrix * self.v1.pos.xyz().extend(1.0);
        let p2 = *matrix * self.v2.pos.xyz().extend(1.0);

        let mut result = *self;

        result.v0.pos = p0;
        result.v1.pos = p1;
        result.v2.pos = p2;

        result
    }

    pub fn reorder(&self, order: VerticesOrder) -> Self {
        match order {
            VerticesOrder::ABC => *self,
            VerticesOrder::ACB => Self::new(self.v0, self.v2, self.v1),
            VerticesOrder::BAC => Self::new(self.v1, self.v0, self.v2),
            VerticesOrder::BCA => Self::new(self.v1, self.v2, self.v0),
            VerticesOrder::CAB => Self::new(self.v2, self.v0, self.v1),
            VerticesOrder::CBA => Self::new(self.v2, self.v1, self.v0),
        }
    }
}

pub struct Mesh {
    pub triangle_indices: Vec<UVec3>,
    pub vertices: Vec<Vertex>,
}

impl Mesh {
    pub fn get_vertices_from_triangle_indices(&self, triangle_indices: UVec3) -> [Vertex; 3] {
        [
            self.vertices[triangle_indices.x as usize],
            self.vertices[triangle_indices.y as usize],
            self.vertices[triangle_indices.z as usize],
        ]
    }
}

pub struct BoundingBox2D {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

pub fn get_triangle_bounding_box_2d(positions: &[Vec2; 3]) -> BoundingBox2D {
    let left = positions[0].x.min(positions[1].x).min(positions[2].x);
    let right = positions[0].x.max(positions[1].x).max(positions[2].x);
    let top = positions[0].y.min(positions[1].y).min(positions[2].y);
    let bottom = positions[0].y.max(positions[1].y).max(positions[2].y);

    BoundingBox2D {
        left,
        right,
        top,
        bottom,
    }
}
