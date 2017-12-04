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

use color::*;
use glapp::{DEFAULT_HEIGHT, DEFAULT_WIDTH};

use na::Transform3;

use std::sync::{Mutex, MutexGuard};

lazy_static! {
    pub static ref SKETCH: Mutex<Sketch> = Mutex::new(Sketch::new());
}

pub struct Sketch {
    pub width: u32,
    pub height: u32,
    pub background: Color,
    pub fill: Color,
    pub stroke: Color,
    pub stroke_weight: u32,
    pub transformation: Transform3<f32>,
}

impl Sketch {
    pub fn new() -> Sketch {
        Sketch {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            background: 1.0.into(),
            fill: (0.0, 1.0, 0.0).into(),
            stroke: 0.0.into(),
            stroke_weight: 1,
            transformation: Transform3::identity(),
        }
    }
}

pub fn get_sketch<'a>() -> MutexGuard<'a, Sketch> {
    SKETCH.lock().unwrap()
}

pub fn stroke_weight(w: u32) {
    SKETCH.lock().unwrap().stroke_weight = w;
}

pub fn get_stroke_weight() -> u32 {
    SKETCH.lock().unwrap().stroke_weight
}
