use wgpu::{BindGroup, BufferUsages, Device, RenderPass, RenderPipelineDescriptor, ShaderModule, ShaderModuleDescriptor, TextureFormat, util::{BufferInitDescriptor, DeviceExt}};
use zerocopy::IntoBytes;

use crate::{graphics::{Graphics, Shaders, Vertex, uniforms::{Binding, Bindings}}, procedural::{circle::Circle, rrect::RRect}};

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

pub struct Renderer<'a> {
    pipeline: wgpu::RenderPipeline,
    instances: wgpu::Buffer,
    number: u32,
    bindgroups: Vec<&'a BindGroup>,
}

impl Renderer<'_> {
    pub fn render(&self, pass: &mut RenderPass) {
        // println!("Rendering {} instances", self.number);
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.instances.slice(..));
        self.bindgroups.iter().enumerate().for_each(|(index, &group)| {
            pass.set_bind_group(index as u32, group, &[]);
        });
        pass.draw(0..4, 0..self.number);
    }
}

pub trait IntoRenderer<I: Vertex> {
    // const SHADER: ShaderModuleDescriptor<'static>;
    const VERTEX: &'static str;
    const FRAGMENT: &'static str;

    fn shader<'a>(&self, shaders: &'a Shaders) -> &'a ShaderModule;
    fn instances(&self) -> &[I];
    fn bind<'a>(&self, bindings: &'a Bindings) -> Vec<&'a Binding>;

    fn renderer<'a>(&self, graphics: &'a Graphics) -> Renderer<'a> {
        let module = self.shader(&graphics.shaders);
        let instances = self.instances();

        let (bind_group_layouts, bind_groups): (Vec<_>, Vec<_>) = self.bind(&graphics.bindings)
            .into_iter()
            .map(|bg| (&bg.layout, &bg.group))
            .unzip();

        let layout = graphics.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &bind_group_layouts,
            immediate_size: 0,
        });

        Renderer {
            pipeline: graphics.device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("Shape Render Pipeline"),
                layout: Some(&layout), // Uniforms would be set here
                vertex: wgpu::VertexState {
                    module,
                    entry_point: Some(Self::VERTEX),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    buffers: &[I::LAYOUT],
                },
                fragment: Some(wgpu::FragmentState {
                    module,
                    entry_point: Some(Self::FRAGMENT),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    targets: &[Some(wgpu::ColorTargetState { // 4.
                        format: graphics.config.format,
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
            instances: graphics.device.create_buffer_init(&BufferInitDescriptor {
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
    fn renderers<'a>(&self, graphics: &'a Graphics) -> Vec<Renderer<'a>>;
}
