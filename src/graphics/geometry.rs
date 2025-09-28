use wgpu::util::BufferInitDescriptor;
use wgpu_macros::VertexLayout;
use zerocopy::{Immutable, IntoBytes};

use super::Bufferize;

pub trait Vertex: IntoBytes + Immutable + VertexLayout {}

impl<'b, V: Vertex> Bufferize<'b> for &'b [V] {
    fn descriptor(&self) -> BufferInitDescriptor<'b> {
        BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: self.as_bytes(),
            usage: wgpu::BufferUsages::VERTEX,
        }
    }
}

impl<'b> Bufferize<'b> for &'b [u16] {
    fn descriptor(&self) -> BufferInitDescriptor<'b> {
        BufferInitDescriptor {
            label: Some("Index buffer"),
            contents: self.as_bytes(),
            usage: wgpu::BufferUsages::INDEX,
        }
    }
}

pub struct Geometry<V: Vertex> {
    pub vertices: Vec<V>,
    pub indices: Vec<u16>,
}