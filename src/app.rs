use std::sync::Arc;

use winit::{application::ApplicationHandler, event::{ElementState, KeyEvent, WindowEvent}, event_loop::ActiveEventLoop, keyboard::{KeyCode, PhysicalKey}, window::Window};

use crate::{graphics::Graphics, procedural::{IntoRenderers, circle::Circle, canvas::Canvas, rrect::RRect}};

#[derive(Default)]
pub enum App {
    #[default]
    Paused,
    Running(Arc<Window>, Graphics, Canvas)
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
        let mut canvas = Canvas::new();
        let circles: Vec<Circle> = (0..300).step_by(50).into_iter().map(|num| {
            Circle { center:[250.0+(num as f32),200.0], radius: 20., thickness: 5., color: [1.0, 0.0, 0.0] }
        }).collect();
        canvas.circles(&circles);
        canvas.rrects(&[RRect { left: 250.0, right: 350.0, top: 300.0, bottom: 350.0, thickness: 10., radius: 20., color: [0.0, 1.0, 0.0]}]);
        *self = Self::Running(window, graphics, canvas);
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
                    App::Running(_, graphics, canvas) => {
                        let renderers = canvas.renderers(graphics);
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
