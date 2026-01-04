use wgpu::include_wgsl;
use wgpu_macros::VertexLayout;
use zerocopy::{Immutable, IntoBytes};

use crate::{graphics::Vertex, procedural::IntoRenderer};


#[derive(IntoBytes, Immutable, VertexLayout)]
#[layout(Instance)]
#[location = 1]
pub struct Rect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub thickness: f32,
    pub color: [f32; 3],
}

impl Vertex for Rect {}

impl IntoRenderer<Rect, ()> for &[Rect] {
    const SHADER: wgpu::ShaderModuleDescriptor<'static> = include_wgsl!("../shaders/rectangle.wgsl");

    fn instances(&self) -> &[Rect] {
        self
    }

    fn uniforms(&self) -> () { }
}
