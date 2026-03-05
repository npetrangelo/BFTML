use wgpu::{Device, include_wgsl};
use wgpu_macros::VertexLayout;
use zerocopy::{Immutable, IntoBytes};

use crate::{graphics::{Vertex, uniforms::Uniforms}, procedural::IntoRenderer};

#[derive(IntoBytes, Immutable, VertexLayout)]
#[layout(Instance)]
#[location = 0]
#[repr(C)]
pub struct Rect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub thickness: f32,
    pub color: [f32; 4],
}

impl Vertex for Rect {}

impl IntoRenderer<Rect> for &[Rect] {
    const SHADER: wgpu::ShaderModuleDescriptor<'static> = include_wgsl!("../shaders/rect.wgsl");

    fn instances(&self) -> &[Rect] {
        self
    }
    
    fn bindings(&self, uniforms: &Uniforms, device: &Device) -> Vec<crate::graphics::uniforms::Binding> {
        vec![uniforms.binding(&[uniforms.size.as_ref(), uniforms.scale.as_ref()], device)]
    }
}
