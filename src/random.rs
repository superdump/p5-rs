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

extern crate noise;
extern crate rand;

use na::Point4;

use random::noise::NoiseModule;
use random::rand::Rng;
use std::sync::Mutex;

lazy_static! {
    pub static ref PERLIN: Mutex<noise::Perlin> = Mutex::new(noise::Perlin::new());
}

pub fn random(l: f32, u: f32) -> f32 {
    rand::thread_rng().gen_range(l, u)
}

pub fn noise(p: Point4<f32>) -> f32 {
    let mut point: [f32; 4] = [0.0; 4];
    point.copy_from_slice(p.coords.as_slice());
    (PERLIN.lock().unwrap().get(point) + 1.0) * 0.5
}
