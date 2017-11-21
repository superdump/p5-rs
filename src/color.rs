/*
 * MIT License
 *
 * Copyright (c) 2017 Robert Swain <robert.swain@gmail.com
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

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

impl Color {
    pub fn as_vec4(&self) -> Vec<f32> {
        vec![self.r, self.g, self.b, self.a]
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(p: (f32, f32, f32, f32)) -> Self {
        Color {
            r: p.0,
            g: p.1,
            b: p.2,
            a: p.3,
        }
    }
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

pub fn get_fill() -> Color {
    SKETCH.lock().unwrap().fill.clone()
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
