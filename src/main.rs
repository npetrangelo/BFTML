use std::fs::File;

use app::App;
use winit::event_loop::{ControlFlow, EventLoop};

mod app;
mod graphics;
mod parser;

fn main() {
    let file = File::open("file.xml").expect("File exists");
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    // let mut state = State::new(window).await;
    event_loop.run_app(&mut App::default()).unwrap();
}
