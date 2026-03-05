use std::marker::PhantomData;

use wgpu::{BindGroup, BindGroupLayout, BindGroupLayoutEntry, BindingType, Buffer, BufferUsages, Device, Queue, ShaderStages, util::{BufferInitDescriptor, DeviceExt}};
use winit::dpi::LogicalSize;
use zerocopy::{Immutable, IntoBytes};

pub struct Layout {
    pub visibility: ShaderStages,
    pub ty: BindingType,
}

impl Layout {
    pub fn bind(&self, binding: u32) -> BindGroupLayoutEntry {
        BindGroupLayoutEntry {
            binding,
            visibility: self.visibility,
            ty: self.ty,
            count: None
        }
    }
}

pub struct Uniform<T: IntoBytes + Immutable> {
    layout: Layout,
    buffer: Buffer,
    _phantom: PhantomData<T>
}

/// This is here for type erasure when creating bindgroups
pub struct UniformRef<'a> {
    pub layout: &'a Layout,
    pub buffer: &'a Buffer,
}

impl<T: IntoBytes + Immutable> Uniform<T> {
    pub fn init(
        data: &T,
        visibility: ShaderStages,
        device: &Device,
    ) -> Self {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: data.as_bytes(),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let layout = Layout {
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
        };
        Self { layout, buffer, _phantom: PhantomData }
    }

    pub fn write(&self, queue: &Queue, data: &T) {
        queue.write_buffer(&self.buffer, 0, data.as_bytes());
    }

    pub fn as_ref(&self) -> UniformRef {
        UniformRef {
            layout: &self.layout,
            buffer: &self.buffer,
        }
    }
}

pub struct Uniforms {
    pub screen: Uniform<[f32; 2]>
}

pub struct Binding {
    pub layout: BindGroupLayout,
    pub group: BindGroup,
}

impl Uniforms {
    pub fn init(device: &Device, size: &LogicalSize<f32>) -> Self {
        println!("Screen size: {}x{}", size.width, size.height);
        Self {
            screen: Uniform::init(
                &[size.width, size.height],
                ShaderStages::VERTEX,
                device,
            ),
        }
    }

    pub fn binding(&self, uniforms: &[UniformRef], device: &Device) -> Binding {
        let (layout_entries, entries): (Vec<_>, Vec<_>) = uniforms
            .iter()
            .enumerate()
            .map(|(i, u)| (
                u.layout.bind(i as u32),
                wgpu::BindGroupEntry {
                    binding: i as u32,
                    resource: u.buffer.as_entire_binding(),
                }
            ))
            .unzip();

        let layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &layout_entries,
            }
        );

        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &entries,
        });

        Binding { layout, group }
    }
}
