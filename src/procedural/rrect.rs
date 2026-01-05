use wgpu::include_wgsl;
use wgpu_macros::VertexLayout;
use zerocopy::{Immutable, IntoBytes};

use crate::{graphics::Vertex, procedural::IntoRenderer};

#[derive(IntoBytes, Immutable, VertexLayout)]
#[layout(Instance)]
#[location = 1]
pub struct RRect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub thickness: f32,
    pub radius: f32,
    pub color: [f32; 3],
}

impl Vertex for RRect {}

impl IntoRenderer<RRect, ()> for &[RRect] {
    const SHADER: wgpu::ShaderModuleDescriptor<'static> = include_wgsl!("../shaders/rrect.wgsl");

    fn instances(&self) -> &[RRect] {
        self
    }

    fn uniforms(&self) -> () { }
}