use wgpu::{Device, include_wgsl};
use wgpu_macros::VertexLayout;
use zerocopy::{Immutable, IntoBytes};

use crate::{graphics::{Vertex, uniforms::Uniforms}, procedural::IntoRenderer};

/**
A point with a distance offset
*/
#[derive(Clone, Debug, IntoBytes, Immutable, VertexLayout)]
#[layout(Instance)]
#[location = 0]
#[repr(C)]
pub struct Circle {
    pub center: [f32; 2],
    pub radius: f32,
    pub thickness: f32,
    pub color: [f32; 4],
}

impl Vertex for Circle {}

impl IntoRenderer<Circle> for &[Circle] {
    const SHADER: wgpu::ShaderModuleDescriptor<'static> = include_wgsl!("../shaders/circle.wgsl");

    fn instances(&self) -> &[Circle] {
        self
    }
    
    fn bindings(&self, uniforms: &Uniforms, device: &Device) -> Vec<crate::graphics::uniforms::Binding> {
        vec![uniforms.binding(&[uniforms.screen.as_ref()], device)]
    }   
}
