use wgpu::{Device, include_wgsl};
use wgpu_macros::VertexLayout;
use zerocopy::{Immutable, IntoBytes};

use crate::{graphics::{Vertex, uniforms::Uniforms}, procedural::IntoRenderer};

#[derive(Clone, IntoBytes, Immutable, VertexLayout)]
#[layout(Instance)]
#[location = 0]
#[repr(C)]
pub struct RRect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub thickness: f32,
    pub radius: f32,
    pub color: [f32; 4],
}

impl Vertex for RRect {}

impl IntoRenderer<RRect> for &[RRect] {
    const SHADER: wgpu::ShaderModuleDescriptor<'static> = include_wgsl!("../shaders/rrect.wgsl");

    fn instances(&self) -> &[RRect] {
        self
    }
    
    fn bindings(&self, uniforms: &Uniforms, device: &Device) -> Vec<crate::graphics::uniforms::Binding> {
        vec![uniforms.binding(&[uniforms.size.as_ref(), uniforms.scale.as_ref()], device)]
    }
}