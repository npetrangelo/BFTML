use wgpu::include_wgsl;
use wgpu_macros::VertexLayout;
use zerocopy::{Immutable, IntoBytes};

use crate::{graphics::{Vertex, uniforms::{Binding, Bindings}}, procedural::IntoRenderer};

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
    
    fn bind<'a>(&self, bindings: &'a Bindings) -> Vec<&'a Binding> {
        vec![&bindings.screen]
    }
}
