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
extern crate rand;

use p5::*;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

const N_OBJECTS: usize = 10_000;

static mut points: Option<Vec<Point>> = None;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn setup() {
    size(WIDTH, HEIGHT);
    background(0.2);

    let between = Range::new(-((WIDTH/2) as f32), (WIDTH/2) as f32);
    let mut rng = rand::thread_rng();
    let mut ps = Vec::with_capacity(N_OBJECTS);
    for _ in 0..N_OBJECTS {
        ps.push(Point::new(
            between.ind_sample(&mut rng),
            between.ind_sample(&mut rng),
            0.0
        ));
    }
    unsafe {
        points = Some(ps);
    }
}

fn draw() {
    fill(1.0);
    unsafe {
        if let Some(ref ps) = points {
            for p in ps {
                triangle(
                    *p,
                    Point::new(p.x + 3.0, p.y + 3.0, p.z),
                    Point::new(p.x + 6.0, p.y, p.z),
                );
            }
        }
    }
}

fn main() {
    run_sketch(setup, draw, true);
}
