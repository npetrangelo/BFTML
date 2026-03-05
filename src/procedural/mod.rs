use wgpu::{BindGroup, BufferUsages, Device, RenderPass, RenderPipelineDescriptor, ShaderModuleDescriptor, TextureFormat, util::{BufferInitDescriptor, DeviceExt}};
use zerocopy::IntoBytes;

use crate::{graphics::{Graphics, Vertex, uniforms::{Binding, Uniforms}}, procedural::{circle::Circle, rrect::RRect}};

pub mod canvas;

pub mod circle;
// pub mod ellipse;
// pub mod square;
pub mod rect;
pub mod rrect;
// pub mod polygon;

pub enum Shapes {
    Circles(Vec<Circle>),
    RRects(Vec<RRect>)
}

pub struct Renderer {
    pipeline: wgpu::RenderPipeline,
    instances: wgpu::Buffer,
    number: u32,
    bindgroups: Vec<BindGroup>,
}

impl Renderer {
    pub fn render(&self, pass: &mut RenderPass) {
        // println!("Rendering {} instances", self.number);
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.instances.slice(..));
        self.bindgroups.iter().enumerate().for_each(|(index, group)| {
            pass.set_bind_group(index as u32, group, &[]);
        });
        pass.draw(0..4, 0..self.number);
    }
}

pub trait IntoRenderer<I: Vertex> {
    const SHADER: ShaderModuleDescriptor<'static>;

    fn instances(&self) -> &[I];
    fn bindings(&self, uniforms: &Uniforms, device: &Device) -> Vec<Binding>;

    fn renderer(&self, device: &Device, uniforms: &Uniforms, format: &TextureFormat) -> Renderer {
        let module = &device.create_shader_module(Self::SHADER);
        let instances = self.instances();

        let (bind_group_layouts, bind_groups): (Vec<_>, Vec<_>) = self.bindings(uniforms, device)
            .into_iter()
            .map(|bg| (bg.layout, bg.group))
            .unzip();

        let layout_refs: Vec<&wgpu::BindGroupLayout> = bind_group_layouts.iter().collect();

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &layout_refs,
            immediate_size: 0,
        });

        Renderer {
            pipeline: device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("Shape Render Pipeline"),
                layout: Some(&layout), // Uniforms would be set here
                vertex: wgpu::VertexState {
                    module,
                    entry_point: Some("vs_main"),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    buffers: &[I::LAYOUT],
                },
                fragment: Some(wgpu::FragmentState {
                    module,
                    entry_point: Some("fs_border"),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    targets: &[Some(wgpu::ColorTargetState { // 4.
                        format: *format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip, // 1.
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw, // 2.
                    cull_mode: None,
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                cache: None,
                multiview_mask: None,
            }),
            instances: device.create_buffer_init(&BufferInitDescriptor {
                label: Some("instances"),
                contents: instances.as_bytes(),
                usage: BufferUsages::VERTEX
            }),
            number: instances.len() as u32,
            bindgroups: bind_groups,
        }
    }
}

pub trait IntoRenderers {
    fn renderers(&self, graphics: &Graphics) -> Vec<Renderer>;
}
