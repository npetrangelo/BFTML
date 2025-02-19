use std::{borrow::Cow, sync::Arc};

use wgpu::{util::DeviceExt, Buffer, RenderPipeline, Surface, SurfaceCapabilities};
use winit::window::Window;

use crate::mesh::{Mesh, VertexLayout};

const BLACK: wgpu::Color = wgpu::Color { r: 0., g: 0., b: 0., a: 1. };

pub struct Buffers {
    vertices: Buffer,
    indices: Buffer,
    num_indices: u32
}

pub struct GPU {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl GPU {
    fn new(instance: &wgpu::Instance, surface: &Surface) -> (Self, SurfaceCapabilities) {
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
                },
                None, // Trace path
            ).await.expect("Device and queue should exist");
            (adapter, device, queue)
        });

        let surface_caps = surface.get_capabilities(&adapter);

        (Self {
            device,
            queue
        },
        surface_caps)
    }

    fn buffers(&self, mesh: &Mesh) -> Buffers {
        let vertices = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(mesh.vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let indices = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(mesh.indices.as_slice()),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        Buffers {
            vertices,
            indices,
            num_indices: mesh.indices.len() as u32,
        }
    }

    pub fn render(&self, surface: &Surface, pipeline: &RenderPipeline, buffers: &Buffers) -> Result<(), wgpu::SurfaceError> {
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
            render_pass.set_vertex_buffer(0, buffers.vertices.slice(..));
            render_pass.set_index_buffer(buffers.indices.slice(..), wgpu::IndexFormat::Uint16); // 1.
            render_pass.draw_indexed(0..buffers.num_indices, 0, 0..1); // 2.
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub struct Graphics {
    surface: wgpu::Surface<'static>,
    gpu: GPU,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
}

impl Graphics {
    pub fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window, so this should be safe.
        let surface= instance.create_surface(window).unwrap();

        let (gpu, surface_caps) = GPU::new(&instance, &surface);

        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&gpu.device, &config);

        Self {
            surface,
            gpu,
            config,
            size,
        }
    }

    pub fn pipeline<V: VertexLayout<'static, 2>>(&self, path: &str) -> RenderPipeline {
        let source = std::fs::read_to_string(path).expect("reading shader failed");
        let shader = self.gpu.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("test"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&source)),
        });

        let layout =
            self.gpu.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        
        return self.gpu.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"), // 1.
                buffers: &[
                    V::desc(),
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: self.config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            // continued ...
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
            cache: None, // 6.
        });
    }

    pub fn render(&self, pipeline: &RenderPipeline, mesh: &Mesh) -> Result<(), wgpu::SurfaceError> {
        let buffers = self.gpu.buffers(mesh);
        self.gpu.render(&self.surface, pipeline, &buffers)
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.gpu.device, &self.config);
        }
    }
}
