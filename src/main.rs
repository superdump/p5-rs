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

extern crate p5;

use p5::*;

const N_OBJECTS: usize = 100_000;

static mut POINTS: Option<Vec<Point3<f32>>> = None;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn setup() {
    size(WIDTH, HEIGHT);
    background(0.2);

    let mut ps: Vec<Point3<f32>> = Vec::with_capacity(N_OBJECTS);
    for _ in 0..N_OBJECTS {
        ps.push(Point3::new(
            random(-((WIDTH / 2) as f32), (WIDTH / 2) as f32),
            random(-((WIDTH / 2) as f32), (WIDTH / 2) as f32),
            0.0,
        ));
    }
    unsafe {
        POINTS = Some(ps);
    }
}

fn draw() {
    stroke((1.0, 1.0, 1.0, 0.3));
    fill((1.0, 1.0, 1.0, 0.3));
    stroke_weight(12);

    unsafe {
        if let Some(ref ps) = POINTS {
            let off1: Vector3<f32> = Vector3::new(12.0, 0.0, 0.0);
            let off2: Vector3<f32> = Vector3::new(6.0, 12.0, 0.0);
            for p in ps {
                triangle(*p, *p + off1, *p + off2);
            }
        }
    }
}

fn main() {
    run_sketch(setup, draw, true);
}
