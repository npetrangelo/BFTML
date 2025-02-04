use std::sync::Arc;

use winit::{application::ApplicationHandler, event::{ElementState, KeyEvent, WindowEvent}, keyboard::{KeyCode, PhysicalKey}, window::Window};


#[derive(Default)]
pub enum App {
    #[default]
    Paused,
    Running(Arc<Window>)
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(
            Window::default_attributes().with_title("Learn WGPU")
        ).unwrap());
        *self = Self::Running(window);
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
            _ => (),
        }
    }
}

