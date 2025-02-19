use std::sync::Arc;

use winit::{application::ApplicationHandler, event::{ElementState, KeyEvent, WindowEvent}, event_loop::ActiveEventLoop, keyboard::{KeyCode, PhysicalKey}, window::Window};

use crate::{graphics::Graphics, mesh::{Mesh, Point}};

#[derive(Default)]
pub enum App {
    #[default]
    Paused,
    Running(Arc<Window>, Graphics, Mesh)
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(
            Window::default_attributes().with_title("Learn WGPU")
        ).unwrap());
        let graphics = Graphics::new(window.clone());
        let mesh = Mesh::default();
        *self = Self::Running(window, graphics, mesh);
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        *self = Self::Paused;
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                    ..
                },
                ..
            } => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                match self {
                    App::Paused => todo!(),
                    App::Running(window, graphics, mesh) => {
                        let pipeline = graphics.pipeline::<Point>("src/test.wgsl");
                        graphics.render(&pipeline, mesh);
                        window.request_redraw();
                    }
                }
            },
            WindowEvent::Resized(physical_size) => {
                match self {
                    App::Paused => todo!(),
                    App::Running(_, graphics, _) => graphics.resize(physical_size),
                }
            },
            _ => (),
        }
    }
}
