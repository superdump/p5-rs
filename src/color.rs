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

use na::Vector4;

use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct Color(Vector4<f32>);

impl Color {
    pub fn as_slice(&self) -> &[f32] {
        self.0.as_slice()
    }
}

impl Deref for Color {
    type Target = Vector4<f32>;
    fn deref(&self) -> &Vector4<f32> {
        &self.0
    }
}
impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Vector4<f32> {
        &mut self.0
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(c: (f32, f32, f32, f32)) -> Color {
        Color(Vector4::new(c.0, c.1, c.2, c.3))
    }
}
impl From<(f32, f32, f32)> for Color {
    fn from(c: (f32, f32, f32)) -> Color {
        Color(Vector4::new(c.0, c.1, c.2, 1.0))
    }
}
impl From<f32> for Color {
    fn from(c: f32) -> Color {
        Color(Vector4::new(c, c, c, 1.0))
    }
}

pub fn draw_background() {
    channel::push(Box::new(move || {
        let background = SKETCH.lock().unwrap().background.clone();
        glapp::background(&background);
    }));
}

pub fn background<C: Into<Color>>(color: C) {
    SKETCH.lock().unwrap().background = color.into();
    draw_background();
}

pub fn fill<C: Into<Color>>(color: C) {
    SKETCH.lock().unwrap().fill = color.into();
}

pub fn get_fill() -> Color {
    SKETCH.lock().unwrap().fill.clone()
}

pub fn noFill() {
    SKETCH.lock().unwrap().fill = (0.0, 0.0, 0.0, 0.0).into();
}

pub fn stroke<C: Into<Color>>(color: C) {
    SKETCH.lock().unwrap().stroke = color.into();
}

pub fn get_stroke() -> Color {
    SKETCH.lock().unwrap().stroke.clone()
}

pub fn noStroke() {
    SKETCH.lock().unwrap().stroke = (0.0, 0.0, 0.0, 0.0).into();
}
