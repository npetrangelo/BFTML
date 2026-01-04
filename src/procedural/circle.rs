use wgpu::include_wgsl;
use wgpu_macros::VertexLayout;
use zerocopy::{Immutable, IntoBytes};

use crate::{graphics::Vertex, procedural::IntoRenderer};

/**
A point with a distance offset
*/
#[derive(IntoBytes, Immutable, VertexLayout)]
#[layout(Instance)]
#[location = 1]
pub struct Circle {
    pub center: [f32; 2],
    pub radius: f32,
    pub thickness: f32,
    pub color: [f32; 3],
}

impl Vertex for Circle {}

impl IntoRenderer<Circle, ()> for &[Circle] {
    const SHADER: wgpu::ShaderModuleDescriptor<'static> = include_wgsl!("../shaders/circle.wgsl");

    fn instances(&self) -> &[Circle] {
        self
    }

    /// Empty function with no return type implicitly returns `()`
    fn uniforms(&self) { }
}
