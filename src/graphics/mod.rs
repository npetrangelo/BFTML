use std::sync::Arc;
use wgpu_macros::VertexLayout;
use winit::{dpi::LogicalSize, window::Window};
use zerocopy::{Immutable, IntoBytes};

use crate::{graphics::uniforms::Uniforms, procedural::{IntoRenderer, Renderer}};

// pub mod bindings;
// pub mod middleware;
pub mod uniforms;

const BLACK: wgpu::Color = wgpu::Color { r: 0., g: 0., b: 0., a: 1. };

pub trait Vertex: IntoBytes + Immutable + VertexLayout {}

pub struct Graphics {
    pub device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    pub uniforms: Uniforms,
}

impl Graphics {
    pub fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window, so this should be safe.
        let size = window.inner_size(); // Grab size before window is moved
        let scale_factor = window.scale_factor();
        let surface = instance.create_surface(window).unwrap();

        let (adapter, device, queue) = pollster::block_on(async {
            let adapter = instance.request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
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
                    experimental_features: wgpu::ExperimentalFeatures::disabled(),
                }).await.expect("Device and queue should exist");
            (adapter, device, queue)
        });

        device.on_uncaptured_error(Arc::new(|error| {
            eprintln!("wgpu error: {}", error);
        }));

        let surface_caps = surface.get_capabilities(&adapter);

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

        surface.configure(&device, &config);
        let logical: LogicalSize<f32> = size.to_logical(scale_factor);
        let uniforms = Uniforms::init(&device, &logical);

        // Check here when using updated wgpu
        // https://github.com/gfx-rs/wgpu/issues/3756
        unsafe {
            if let Some(hal_surface) = surface.as_hal::<wgpu::hal::api::Metal>() {
                let guard = hal_surface.render_layer().lock();
                guard.set_presents_with_transaction(true);
            }
        }

        Self {
            device,
            queue,
            surface,
            config,
            uniforms
        }
    }

    pub fn renderer<I: Vertex, T: IntoRenderer<I>>(&self, t: T) -> Renderer {
        t.renderer(&self.device, &self.uniforms, &self.config.format)
    }

    pub fn render(&self, renderers: &[Renderer]) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    // depth_slice: None, uncomment when wgpu 26 compiles
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });
            for renderer in renderers {
                renderer.render(&mut pass);
            }
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, scale_factor: f64) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            let logical: winit::dpi::LogicalSize<f32> = new_size.to_logical(scale_factor);
            self.uniforms.screen.write(&self.queue, &[logical.width, logical.height]);
        }
    }
}
