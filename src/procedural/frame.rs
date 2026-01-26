use crate::{graphics::Graphics, procedural::{IntoRenderers, Renderer, circle::Circle, rrect::RRect}};

enum Shapes {
    Circles(Vec<Circle>),
    RRects(Vec<RRect>)
}

// What I would really like here, is a collection of renderables - that's proving difficult because I can't implement IntoRenderer directly on an enum.

pub struct Frame {
    layers: Vec<Shapes>
}

impl Frame {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
        }
    }

    pub fn circles(&mut self, circles: &[Circle]) {
        self.layers.push(Shapes::Circles(circles.iter().cloned().collect()));
    }

    pub fn rrects(&mut self, rrects: &[RRect]) {
        self.layers.push(Shapes::RRects(rrects.iter().cloned().collect()));
    }
}

impl IntoRenderers for Frame {
    fn renderers(&self, graphics: &Graphics) -> Vec<Renderer> {
        self.layers.iter().map(|layer| {
            match layer {
                Shapes::Circles(circles) => graphics.renderer(circles.as_slice()),
                Shapes::RRects(rrects) => graphics.renderer(rrects.as_slice()),
            }
        }).collect()
    }
}