use super::{geometry::{Geometry, Vertex}, Bindings, Bufferize, BLACK};
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BindGroup, BindGroupEntry, BindGroupLayout, Buffer, PipelineLayout, RenderPipeline, Surface, SurfaceCapabilities};

pub struct GeoBuffers {
    vertices: Buffer,
    indices: Buffer,
    num_indices: u32,
}

/**
`Device` and `Queue` are naturally coupled because they are created together.
Most of the actual doing things is done by them both, so combined, we have a GPU
object that turns the various descriptors into their respective actual components,
and then acts on them.

This coupling also allows us to hide a lot of complexity in simpler abstractions without
losing any functionality.
*/
pub struct GPU {
    pub device: wgpu::Device,
    queue: wgpu::Queue,
}

impl GPU {
    /**
    Creates the `Adapter`, and uses it to create the `Device` and `Queue`.
    This is done in `async` because in theory, wgpu wants us to be able to keep doing things
    while it assesses your hardware. We don't really care though, so we contain the `async` here.

    Also creates the `SurfaceCapabilities`, because that is the only other thing `Adapter` is needed for.
    That means we can drop the `Adapter` at the end of this function.
    This also conforms with the WebGPU spec, which says an adapter should not be used to create more than one `Device`.
    Every invocation of `new()` creates, and then drops, a new `Adapter`.
    */
    pub fn new(instance: &wgpu::Instance, surface: &Surface) -> (Self, SurfaceCapabilities) {
        let (adapter, device, queue) = pollster::block_on(async {
            let adapter = instance.request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(surface),
                    force_fallback_adapter: false,
                },
            ).await.expect("Adapter should exist");

            let (device, queue) = adapter.request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: wgpu::MemoryHints::Performance,
                    trace: wgpu::Trace::Off,
                }).await.expect("Device and queue should exist");
            (adapter, device, queue)
        });

        let surface_caps = surface.get_capabilities(&adapter);

        (Self {
            device,
            queue
        },
        surface_caps)
    }

    /**
    Creates an iterator that will generate buffers for every `BufferInitDescriptor` in the slice.
    */
    pub fn bufferize<'a>(&self, buffers: &'a [BufferInitDescriptor<'a>]) -> impl Iterator<Item = Buffer> + use<'_,'a> {
        buffers.iter().map(|buffer: &BufferInitDescriptor<'a>| {
            self.device.create_buffer_init(buffer)
        })
    }

    pub fn geo_buffers<V: Vertex>(&self, geometry: &Geometry<V>) -> GeoBuffers {
        let vertices = self.device.create_buffer_init(&geometry.vertices.descriptor());
        let indices = self.device.create_buffer_init(&geometry.indices.descriptor());

        GeoBuffers {
            vertices,
            indices,
            num_indices: geometry.indices.len() as u32,
        }
    }

    /**
    Creates `PipelineLayout` and all the `BindGroup`s for the pipeline.
    
    The `PipelineLayout` needs to know the layouts of all the `BindGroup`s,
    so it makes to couple them here.
    
    Push constants are not compatible with WGSL, so we will leave that empty.
    */
    pub fn prepare_groups(&self, bind_group_descriptors: &[Bindings], label: Option<&str>) -> (PipelineLayout, Vec<BindGroup>) {
        let (layouts, groups): (Vec<BindGroupLayout>, Vec<BindGroup>) =
            bind_group_descriptors.iter().map(|descriptor|
                self.create_bind_group(descriptor, None)).unzip();
        
        let bind_group_layouts: Vec<&BindGroupLayout> = layouts.iter().map(|l| &*l).collect();
        let pipeline_layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: label.map(|s| format!("{s}_pipeline_layout")).as_deref(),
            bind_group_layouts: &bind_group_layouts,
            push_constant_ranges: &[],
        });

        (pipeline_layout, groups)
    }

    /**
    Creates a `BindGroup` and its layout. Creates buffers as an intermediate step.
    
    A `BindGroup` can have several buffers referenced in it, each at a different binding.
    Binding numbers are determined by the order of `BindGroupEntry`s in the `layouts` Vec.
    */
    pub fn create_bind_group(&self, descriptor: &Bindings, label: Option<&str>) -> (BindGroupLayout, BindGroup) {
        let layout = self.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
            entries: &descriptor.layouts,
            label: label.map(|s| format!("{s}_layout")).as_deref(),
        });

        let buffers: Vec<Buffer> = self.bufferize(&descriptor.buffers).collect();
        let entries: Vec<BindGroupEntry> = buffers.iter().zip(0..buffers.len()).map(|(buffer, binding)| {
            BindGroupEntry {
                binding: binding as u32,
                resource: buffer.as_entire_binding(),
            }
        }).collect();

        (layout.clone(), self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &entries,
            label,
        }))
    }

    /**
    The main event happens here. Given a `Surface` to draw on, a `RenderPipeline` to tell the GPU the shape of the data
    and what to do with it, and finally the data itself between `GeoBuffers` and the `BindGroup` slice,
    the GPU creates a `CommandEncoder` to create a `RenderPass`. The pass is configured with the `RenderPipeline` and `Buffer`s,
    and a draw function is called. The encoder returns a `CommandBuffer` when finished, which is submitted to the `Queue`.

    GeoBuffers are drawn sequentially, first to last.
    */
    pub fn render(&self, surface: &Surface, pipeline: &RenderPipeline, bind_groups: &[BindGroup], geos: &[GeoBuffers]) -> Result<(), wgpu::SurfaceError> {
        let output = surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            render_pass.set_pipeline(pipeline);
            bind_groups.iter().zip(0..bind_groups.len()).for_each(|(group, index)| {
                render_pass.set_bind_group(index as u32, group, &[]);
            });
            for geo in geos.iter() {
                render_pass.set_vertex_buffer(0, geo.vertices.slice(..));
                render_pass.set_index_buffer(geo.indices.slice(..), wgpu::IndexFormat::Uint16); // 1.
                render_pass.draw_indexed(0..geo.num_indices, 0, 0..1); // 2.
            }
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}