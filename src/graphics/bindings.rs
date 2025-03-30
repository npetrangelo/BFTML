use wgpu::{util::BufferInitDescriptor, BindGroupLayoutEntry, ShaderStages};

use super::Bufferize;

#[derive(Clone, Copy)]
pub enum Buffer<T> {
    UNIFORM(T),
    STORAGE(T)
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::core::mem::size_of::<T>(),
    )
}

impl<T> Bufferize for Buffer<T> {
    fn descriptor(&self) -> BufferInitDescriptor {
        match self {
            Buffer::UNIFORM(b) => {
                BufferInitDescriptor {
                    label: Some("Uniform buffer"),
                    contents: unsafe { any_as_u8_slice(b) },
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                }
            },
            Buffer::STORAGE(b) => {
                BufferInitDescriptor {
                    label: Some("Storage buffer"),
                    contents: unsafe { any_as_u8_slice(b) },
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                }
            },
        }
    }
}

pub trait Binding {
    fn bind(&self, binding: u32, visibility: ShaderStages) -> BindGroupLayoutEntry;
}

impl<T> Binding for Buffer<T> {
    fn bind(&self, binding: u32, visibility: ShaderStages) -> BindGroupLayoutEntry {
        use wgpu::BufferBindingType::*;
        let ty = match self {
            Buffer::UNIFORM(_) => Uniform,
            Buffer::STORAGE(_) => Storage { read_only: false },
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

impl<'a> Bindings<'a> {
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
    pub fn bind<T>(&mut self, buffer: &'a Buffer<T>, visibility: ShaderStages) {
        self.layouts.push(buffer.bind(self.layouts.len() as u32, visibility));
        self.buffers.push(buffer.descriptor());
    }
}
