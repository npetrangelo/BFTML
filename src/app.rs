use std::{slice, sync::Arc};

use winit::{application::ApplicationHandler, event::{ElementState, KeyEvent, WindowEvent}, event_loop::ActiveEventLoop, keyboard::{KeyCode, PhysicalKey}, window::Window};

use crate::{elements::rect::{Point, Rect}, graphics::{geometry::Geometry, Graphics, Material}};

#[derive(Default)]
pub enum App {
    #[default]
    Paused,
    Running(Arc<Window>, Graphics, Vec<Geometry<Point>>)
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(
            Window::default_attributes().with_title("Learn WGPU")
        ).unwrap());
        let graphics = Graphics::new(window.clone());
        let rect1 = Rect {
            center: (0., 0.),
            size: (0.75, 0.5),
        };
        let rect2 = Rect {
            center: (0., 0.),
            size: (0.5, 0.75),
        };
        *self = Self::Running(window, graphics, vec![rect1.into(), rect2.into()]);
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
                    App::Running(window, graphics, geometry) => {
                        let material = Material::new("src/shaders/test.wgsl");
                        let (pipeline, groups) = graphics.pipeline::<Point>(&material);
                        let _ = graphics.render::<Point>(&pipeline, groups, &geometry);
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
