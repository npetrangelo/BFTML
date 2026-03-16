use wgpu_macros::VertexLayout;
use zerocopy::{Immutable, IntoBytes};

use crate::{graphics::{Vertex, uniforms::{Binding, Bindings}}, procedural::IntoRenderer};

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
    const VERTEX: &'static str = "vs_main";
    const FRAGMENT: &'static str = "fs_fill";

    fn shader<'a>(&self, shaders: &'a crate::graphics::Shaders) -> &'a wgpu::ShaderModule {
        &shaders.circle
    }

    fn instances(&self) -> &[Circle] {
        self
    }
    
    fn bind<'a>(&self, bindings: &'a Bindings) -> Vec<&'a Binding> {
        vec![&bindings.screen]
    }
}
