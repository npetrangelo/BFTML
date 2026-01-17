use std::collections::HashMap;

use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BindGroup, BufferUsages, Device, PipelineLayoutDescriptor, RenderPass, RenderPipelineDescriptor, ShaderModuleDescriptor, TextureFormat};
use wgpu_macros::VertexLayout;
use zerocopy::{Immutable, IntoBytes};

use crate::graphics::Vertex;

// pub mod frame;

// pub mod point;
// pub mod line;
pub mod circle;
// pub mod ellipse;
// pub mod square;
pub mod rect;
pub mod rrect;
// pub mod polygon;

pub trait Uniforms {
    fn pipeline_layout(&self) -> Option<PipelineLayoutDescriptor>;
    fn bindgroups(&self, device: &Device) -> HashMap<u32, BindGroup>;
}

/// This indicates that there are no uniforms. There is no layout, and there are no bindgroups.
impl Uniforms for () {
    /// There is no `PipelineLayout` for the `RenderPipeline`.
    fn pipeline_layout(&self) -> Option<wgpu::PipelineLayoutDescriptor> {
        None
    }

    /// When `render()` loops over the empty hashmap, it doesn't bind anything.
    fn bindgroups(&self, device: &Device) -> HashMap<u32, wgpu::BindGroup> {
        HashMap::new()
    }
}

pub struct Renderer {
    pipeline: wgpu::RenderPipeline,
    vertices: wgpu::Buffer,
    instances: wgpu::Buffer,
    number: u32,
    bindgroups: HashMap<u32, BindGroup>,
}

impl Renderer {
    pub fn render(&self, pass: &mut RenderPass) {
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.set_vertex_buffer(1, self.instances.slice(..));
        self.bindgroups.iter().for_each(|(&index, group)| {
            pass.set_bind_group(index, group, &[]);
        });
        pass.draw(0..4, 0..self.number);
    }
}

pub trait IntoRenderer<I: Vertex, U: Uniforms> {
    const SHADER: ShaderModuleDescriptor<'static>;

    fn vertices(&self) -> Vec<Pos> {
        BBox {
            center: (0., 0.),
            size: (1., 1.),
        }.into()
    }

    fn instances(&self) -> &[I];
    fn uniforms(&self) -> U;

    fn renderer(&self, device: &Device, format: &TextureFormat) -> Renderer {
        let module = &device.create_shader_module(Self::SHADER);
        let vertices = self.vertices();
        let instances = self.instances();
        let uniforms = self.uniforms();

        let layout = uniforms.pipeline_layout().map(|desc| device.create_pipeline_layout(&desc));

        Renderer {
            pipeline: device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("Shape Render Pipeline"),
                layout: layout.as_ref(), // Uniforms would be set here
                vertex: wgpu::VertexState {
                    module,
                    entry_point: Some("vs_main"),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    buffers: &[Pos::LAYOUT, I::LAYOUT],
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
                    front_face: wgpu::FrontFace::Cw, // 2.
                    cull_mode: Some(wgpu::Face::Back),
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
            vertices: device.create_buffer_init(&BufferInitDescriptor {
                label: Some("circle vertices"),
                contents: vertices.as_bytes(),
                usage: BufferUsages::VERTEX,
            }),
            instances: device.create_buffer_init(&BufferInitDescriptor {
                label: Some("circle instances"),
                contents: instances.as_bytes(),
                usage: BufferUsages::VERTEX
            }),
            number: instances.len() as u32,
            bindgroups: uniforms.bindgroups(device),
        }
    }
}

// #[repr(C)]
#[derive(IntoBytes, Immutable, VertexLayout)]
pub struct Pos([f32; 3]);

impl Vertex for Pos {}

pub struct BBox {
    pub center: (f32, f32),
    pub size: (f32, f32)
}

impl From<BBox> for Vec<Pos> {
    fn from(value: BBox) -> Self {
        let (x, y) = value.center;
        let (w, h) = (value.size.0, value.size.1);
        vec![
            Pos([x - w, y + h, 0.0]),
            Pos([x + w, y + h, 0.0]),
            Pos([x - w, y - h, 0.0]),
            Pos([x + w, y - h, 0.0]),
        ]
    }
}
