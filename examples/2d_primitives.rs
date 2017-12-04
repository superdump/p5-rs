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

static mut T: f32 = 0.0;

fn setup() {
    size(400, 400);
    background(0.2);
}

const RADIUS: f32 = 100.0;
fn draw() {
    let radius_offset: Vector3<f32> = Vector3::new(RADIUS, 0.0, 0.0);

    let p1: Point3<f32> = Point3::new(-50.0, -50.0, 0.0);
    let p2: Point3<f32> = Point3::new(0.0, 43.0, 0.0);
    let p3: Point3<f32> = Point3::new(50.0, -50.0, 0.0);
    let tl: Point3<f32> = Point3::new(-50.0, 50.0, 0.0);
    let br: Point3<f32> = Point3::new(50.0, -50.0, 0.0);

    fill((1.0, 0.0, 0.0));
    push_matrix();
    translate(&radius_offset);
    unsafe {
        rotate(T);
    }
    triangle(p1, p2, p3);
    pop_matrix();

    fill((0.0, 1.0, 0.0));
    push_matrix();
    let sin;
    let cos;
    unsafe {
        sin = (T + std::f32::consts::FRAC_PI_2).sin();
        cos = (T + std::f32::consts::FRAC_PI_2).sin();
    }
    translate(&Vector3::new(RADIUS * sin, RADIUS * cos, 0.0));
    ellipse(Point3::origin(), 200.0, 100.0);
    pop_matrix();

    fill((0.0, 0.0, 1.0));
    push_matrix();
    translate(&radius_offset);
    unsafe {
        rotate(T + std::f32::consts::PI);
    }
    rect(tl, br);
    pop_matrix();

    stroke_weight(10);
    stroke((1.0, 1.0, 0.0));

    push_matrix();
    unsafe {
        rotate(0.5 * T);
    }
    line(
        Point3::new(-0.75 * RADIUS, 0.0, 0.0),
        Point3::new(0.75 * RADIUS, 0.0, 0.0),
    );
    pop_matrix();

    stroke((0.0, 1.0, 1.0));
    point(Point3::origin());

    unsafe {
        T += 0.03;
    }
}

fn main() {
    run_sketch(setup, draw, true);
}
