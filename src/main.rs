use std::fs::File;

use app::App;
use graphics::Vertex;
use wgpu::VertexAttribute;
use winit::event_loop::{ControlFlow, EventLoop};

mod app;
mod graphics;
mod mesh;
mod parser;

fn main() {
    let file = File::open("file.xml").expect("File exists");
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut App::default()).unwrap();
}
