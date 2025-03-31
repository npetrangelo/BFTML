use bytemuck::NoUninit;
use wgpu::util::BufferInitDescriptor;
use wgpu_macros::VertexLayout;

use super::Bufferize;

pub trait Vertex: NoUninit + VertexLayout {}

impl<V: Vertex> Bufferize for Vec<V> {
    fn descriptor(&self) -> BufferInitDescriptor {
        BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: bytemuck::cast_slice(self),
            usage: wgpu::BufferUsages::VERTEX,
        }
    }
}

impl Bufferize for Vec<u16> {
    fn descriptor(&self) -> BufferInitDescriptor {
        BufferInitDescriptor {
            label: Some("Index buffer"),
            contents: bytemuck::cast_slice(self),
            usage: wgpu::BufferUsages::INDEX,
        }
    }
}

pub struct Geometry<V: Vertex> {
    pub vertices: Vec<V>,
    pub indices: Vec<u16>,
}