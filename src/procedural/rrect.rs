use wgpu_macros::VertexLayout;
use zerocopy::{Immutable, IntoBytes};

use crate::{graphics::{Vertex, uniforms::{Binding, Bindings}}, procedural::IntoRenderer};

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
    const VERTEX: &'static str = "vs_main";
    const FRAGMENT: &'static str = "fs_border";

    fn shader<'a>(&self, shaders: &'a crate::graphics::Shaders) -> &'a wgpu::ShaderModule {
        &shaders.rrect
    }

    fn instances(&self) -> &[RRect] {
        self
    }
    
    fn bind<'a>(&self, bindings: &'a Bindings) -> Vec<&'a Binding> {
        vec![&bindings.screen]
    }
}
