use wgpu_macros::VertexLayout;
use crate::graphics::geometry::{Geometry, Vertex};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, VertexLayout)]
pub struct Point {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex for Point {}

pub struct Rect {
    pub center: (f32, f32),
    pub size: (f32, f32)
}

impl From<Rect> for Geometry<Point> {
    fn from(value: Rect) -> Self {
        let (x, y) = value.center;
        let (w, h) = (value.size.0/2., value.size.1/2.);
        Self {
            vertices: vec![
                Point { position: [x - w, y + h, 0.0], color: [0.5, 0.5, 0.0] }, // B
                Point { position: [x + w, y + h, 0.0], color: [1.0, 0.0, 0.0] }, // A
                Point { position: [x - w, y - h, 0.0], color: [0.0, 1.0, 0.0] }, // C
                Point { position: [x + w, y - h, 0.0], color: [0.0, 0.0, 1.0] }, // E
            ],
            indices: vec![0, 1, 2, 3,]
        }
    }
}