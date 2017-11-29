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

static mut t: f32 = 0.0;
static p1: Point = Point {
    x: -50.0,
    y: -50.0,
    z: 0.0,
};
static p2: Point = Point {
    x: 0.0,
    y: 43.0,
    z: 0.0,
};
static p3: Point = Point {
    x: 50.0,
    y: -50.0,
    z: 0.0,
};
static tl: Point = Point {
    x: -50.0,
    y: 50.0,
    z: 0.0,
};
static br: Point = Point {
    x: 50.0,
    y: -50.0,
    z: 0.0,
};

fn setup() {
    size(400, 400);
    background(0.2);
}

fn point_on_circle(center: &Point, radius: f32, sin: f32, cos: f32) -> Point {
    Point {
        x: center.x + radius * sin,
        y: center.y + radius * cos,
        z: center.z,
    }
}

fn draw() {
    let origin: Point = (0.0, 0.0).into();
    let radius: f32 = 100.0;
    let mut sin: f32;
    let mut cos: f32;

    unsafe {
        sin = t.sin();
        cos = t.cos();
    }
    fill((1.0, 0.0, 0.0));
    pushMatrix();
    let triCenter = point_on_circle(&origin, radius, sin, cos);
    translate(triCenter);
    triangle(p1, p2, p3);
    popMatrix();

    unsafe {
        sin = (t + std::f32::consts::FRAC_PI_2).sin();
        cos = (t + std::f32::consts::FRAC_PI_2).sin();
    }
    fill((0.0, 1.0, 0.0));
    pushMatrix();
    let ellipseCenter = point_on_circle(&origin, radius, sin, cos);
    translate(ellipseCenter);
    ellipse(origin, 200.0, 100.0);
    popMatrix();

    unsafe {
        sin = (t + std::f32::consts::PI).sin();
        cos = (t + std::f32::consts::PI).cos();
    }
    fill((0.0, 0.0, 1.0));
    pushMatrix();
    let rectCenter = point_on_circle(&origin, radius, sin, cos);
    translate(rectCenter);
    rect(tl, br);
    popMatrix();

    strokeWeight(10);
    stroke((1.0, 1.0, 0.0));

    pushMatrix();
    unsafe {
        rotate(0.5 * t);
    }
    line((-0.75 * radius, 0.0), (0.75 * radius, 0.0));
    popMatrix();

    stroke((0.0, 1.0, 1.0));
    point(origin);

    unsafe {
        t += 0.03;
    }
}

fn main() {
    run_sketch(setup, draw, true);
}
