use wgpu::{util::BufferInitDescriptor, BindGroupLayoutEntry, ShaderStages};
use zerocopy::{Immutable, IntoBytes};

use super::Bufferize;

#[derive(Clone, Copy)]
pub struct Buffer<'b, T: IntoBytes + Immutable + 'b> {
    pub data: &'b T,
    pub usage: BufferUsage,
}

#[derive(Clone, Copy)]
pub enum BufferUsage {
    UNIFORM,
    STORAGE
}

impl<'b, T: IntoBytes + Immutable + 'b> Bufferize<'b> for Buffer<'b, T> {
    fn descriptor(&self) -> BufferInitDescriptor<'b> {
        use wgpu::BufferUsages;
        let (label, usage) = match self.usage {
            BufferUsage::UNIFORM => (Some("Uniform buffer"), BufferUsages::UNIFORM),
            BufferUsage::STORAGE => (Some("Storage buffer"), BufferUsages::STORAGE),
        };
        BufferInitDescriptor { label, contents: self.data.as_bytes(), usage: usage | BufferUsages::COPY_DST }
    }
}

pub trait Binding {
    fn bind(&self, binding: u32, visibility: ShaderStages) -> BindGroupLayoutEntry;
}

impl<'b, T: IntoBytes + Immutable + 'b> Binding for Buffer<'b, T> {
    fn bind(&self, binding: u32, visibility: ShaderStages) -> BindGroupLayoutEntry {
        use wgpu::BufferBindingType::*;
        let ty = match self.usage {
            BufferUsage::UNIFORM => Uniform,
            BufferUsage::STORAGE => Storage { read_only: false },
        };
        BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }
}

/**
Couples `BindGroupLayoutEntry`s with `BufferInitDescriptor`s for bound buffers.
This is a natural coupling because each buffer to be bound has a corresponding
`BindGroupLayoutEntry` and `BufferInitDescriptor`.
*/
pub struct Bindings<'a> {
    pub layouts: Vec<BindGroupLayoutEntry>,
    pub buffers: Vec<BufferInitDescriptor<'a>>,
}

impl<'b> Bindings<'b> {
    pub fn new() -> Self {
        Self {
            layouts: Vec::new(),
            buffers: Vec::new()
        }
    }

    /**
    Adds binding data for any arbitrary type. Layout entries and buffer descriptors stay aligned.
    
    The generic `T` here means any type can be passed in, and it will be converted to a `BindGroupLayoutEntry`
    and `BufferInitDescriptor` which get pushed to their respective vecs.

    Doing it this way lets the `BufferGroupDescriptor` accept uniform and storage buffers of different types into
    the same vecs, because they will all yield a `BindGroupLayoutEntry` and `BufferInitDescriptor` regardless. 
    */
    pub fn bind<T: IntoBytes + Immutable + 'b>(&mut self, buffer: Buffer<'b, T>, visibility: ShaderStages) {
        self.layouts.push(buffer.bind(self.layouts.len() as u32, visibility));
        self.buffers.push(buffer.descriptor());
    }
}
