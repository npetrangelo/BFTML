use wgpu_macros::*;
use zerocopy::{Immutable, IntoBytes};

use crate::graphics::geometry::{Geometry, Vertex};

#[repr(C)]
#[derive(Copy, Clone, Debug, IntoBytes, Immutable, VertexLayout)]
pub struct Point {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex for Point {}

pub fn pentagon() -> Geometry<Point> {
    Geometry {
        vertices: vec![
            Point { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.5, 0.0] }, // B
            Point { position: [-0.0868241, 0.49240386, 0.0], color: [1.0, 0.0, 0.0] }, // A
            Point { position: [-0.21918549, -0.44939706, 0.0], color: [0.0, 1.0, 0.0] }, // C
            Point { position: [0.44147372, 0.2347359, 0.0], color: [0.0, 0.0, 1.0] }, // E
            Point { position: [0.35966998, -0.3473291, 0.0], color: [0.0, 0.5, 0.5] }, // D
        ],
        indices: vec![0, 1, 2, 3, 4,]
    }
}
