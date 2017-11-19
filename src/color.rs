use channel;
use glapp;
use sketch::*;

#[derive(Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub fn draw_background() {
    channel::send_closure(Box::new(move || {
        glapp::background(&SKETCH.lock().unwrap().background.clone());
    }));
}

pub fn background(color: Color) {
    SKETCH.lock().unwrap().background = color.clone();
    draw_background();
}

pub fn fill(color: Color) {
    SKETCH.lock().unwrap().fill = color;
}

pub fn no_fill() {
    SKETCH.lock().unwrap().fill = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
}

pub fn stroke(color: Color) {
    SKETCH.lock().unwrap().stroke = color;
}

pub fn no_stroke() {
    SKETCH.lock().unwrap().stroke = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
}
