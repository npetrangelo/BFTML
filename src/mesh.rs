use wgpu::VertexAttribute;

use crate::graphics::Buffers;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Point {
    position: [f32; 3],
    color: [f32; 3],
}

/**
 * TODO: Write derive
 */
pub trait VertexLayout<'a, const SIZE: usize>: bytemuck::Pod {
    const ATTRIBS: [wgpu::VertexAttribute; SIZE];

    fn desc() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

impl VertexLayout<'static, 2> for Point {
    const ATTRIBS: [VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
}

pub struct Mesh {
    pub vertices: Vec<Point>,
    pub indices: Vec<u16>
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
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
}
