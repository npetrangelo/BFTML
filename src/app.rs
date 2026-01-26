use std::sync::Arc;

use winit::{application::ApplicationHandler, event::{ElementState, KeyEvent, WindowEvent}, event_loop::ActiveEventLoop, keyboard::{KeyCode, PhysicalKey}, window::Window};

use crate::{graphics::Graphics, procedural::{IntoRenderers, circle::Circle, frame::Frame, rrect::RRect}};

#[derive(Default)]
pub enum App {
    #[default]
    Paused,
    Running(Arc<Window>, Graphics, Frame)
}

impl ApplicationHandler for App {
    // fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: winit::event::StartCause) {
    //     println!("New events");
    // }

    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(
            Window::default_attributes().with_title("Learn WGPU")
        ).unwrap());
        let graphics = Graphics::new(window.clone());
        let mut frame = Frame::new();
        frame.circles(&[Circle { center:[250.0,250.0], radius: 225., thickness: 25., color: [1.0, 0.0, 0.0] }]);
        frame.rrects(&[RRect { left: 250.0, right: 350.0, top: 300.0, bottom: 350.0, thickness: 10., radius: 20., color: [0.0, 1.0, 0.0]}]);
        *self = Self::Running(window, graphics, frame);
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
                    App::Running(_, graphics, frame) => {
                        let renderers = frame.renderers(graphics);
                        graphics.render(&renderers);
                    }
                }
            },
            WindowEvent::Resized(physical_size) => {

                match self {
                    App::Paused => todo!(),
                    App::Running(_, graphics, _) => {
                        graphics.resize(physical_size);
                    }
                }
            },
            _ => (),
        }
    }
}
