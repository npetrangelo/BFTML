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
    const VERTEX: &'static str = "vs_main";
    const FRAGMENT: &'static str = "fs_fill";

    fn shader<'a>(&self, shaders: &'a crate::graphics::Shaders) -> &'a wgpu::ShaderModule {
        &shaders.rect
    }

    fn instances(&self) -> &[Rect] {
        self
    }
    
    fn bind<'a>(&self, bindings: &'a Bindings) -> Vec<&'a Binding> {
        vec![&bindings.screen]
    }
}
