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

use ordered_float;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct HashableColor {
    pub r: ordered_float::OrderedFloat<f32>,
    pub g: ordered_float::OrderedFloat<f32>,
    pub b: ordered_float::OrderedFloat<f32>,
    pub a: ordered_float::OrderedFloat<f32>,
}

impl HashableColor {
    pub fn as_vec4(&self) -> Vec<ordered_float::OrderedFloat<f32>> {
        vec![self.r, self.g, self.b, self.a]
    }
}

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
    pub fn as_hashable(&self) -> HashableColor {
        HashableColor {
            r: self.r.into(),
            g: self.g.into(),
            b: self.b.into(),
            a: self.a.into(),
        }
    }
}

impl From<HashableColor> for Color {
    fn from(c: HashableColor) -> Color {
        Color {
            r: c.r.into(),
            g: c.g.into(),
            b: c.b.into(),
            a: c.a.into(),
        }
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

impl From<(f32, f32, f32)> for Color {
    fn from(p: (f32, f32, f32)) -> Self {
        Color {
            r: p.0,
            g: p.1,
            b: p.2,
            a: 1.0,
        }
    }
}

impl From<f32> for Color {
    fn from(p: f32) -> Self {
        Color {
            r: p,
            g: p,
            b: p,
            a: 1.0,
        }
    }
}

pub fn draw_background() {
    channel::push(Box::new(move || {
        glapp::background(&SKETCH.lock().unwrap().background.clone());
    }));
}

pub fn background<C: Into<Color>>(color: C) {
    SKETCH.lock().unwrap().background = color.into().clone();
    draw_background();
}

pub fn fill<C: Into<Color>>(color: C) {
    SKETCH.lock().unwrap().fill = color.into();
}

pub fn get_fill() -> Color {
    SKETCH.lock().unwrap().fill.clone()
}

pub fn noFill() {
    SKETCH.lock().unwrap().fill = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
}

pub fn stroke<C: Into<Color>>(color: C) {
    SKETCH.lock().unwrap().stroke = color.into();
}

pub fn get_stroke() -> Color {
    SKETCH.lock().unwrap().stroke.clone()
}

pub fn noStroke() {
    SKETCH.lock().unwrap().stroke = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
}
