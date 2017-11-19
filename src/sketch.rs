use std::sync::Mutex;

use color::*;
use glapp::{DEFAULT_WIDTH, DEFAULT_HEIGHT};

lazy_static! {
    pub static ref SKETCH: Mutex<Sketch> = Mutex::new(Sketch::new());
}

pub struct Sketch {
    pub width: u32,
    pub height: u32,
    pub background: Color,
    pub fill: Color,
    pub stroke: Color,
}

impl Sketch {
    pub fn new() -> Sketch {
        Sketch {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            background: Color{ r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
            fill: Color{ r: 0.0, g: 1.0, b: 0.0, a: 1.0 },
            stroke: Color{ r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
        }
    }
}
