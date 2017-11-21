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

use point::*;
use shape;
use shape::Shape;

use std::f32::consts::*;

const N_SEGMENTS: u32 = 64;

pub fn ellipse<P: Into<Point>>(center: P, width: f32, height: f32) {
    Ellipse::new(center.into(), width, height, N_SEGMENTS).draw();
}

pub struct Ellipse {
    points: Vec<Point>,
    indices: Vec<Vec<u32>>,
}

impl Ellipse {
    pub fn new(center: Point, width: f32, height: f32, n_segments: u32) -> Ellipse {
        let mut points: Vec<Point> = Vec::new();
        let mut indices: Vec<Vec<u32>> = Vec::new();

        let a = width * 0.5;
        let b = height * 0.5;

        let point_from_angle = |angle: f32| -> Point {
            Point {
                x: a * angle.cos(),
                y: b * angle.sin(),
                z: 0.0,
            }
        };

        points.push(center.clone());
        points.push(center + point_from_angle(0.0f32));
        let da = 2.0 * PI / (n_segments as f32);
        for i in 1..n_segments/2 {
            let p = point_from_angle((i as f32) * da);
            points.push(center + Point {x: p.x, y: -p.y, z: p.z});
            points.push(center + p);
        }
        points.push(center + point_from_angle(PI));

        /* e.g.
         * n_segments = 6
         * always anti-clockwise through vertices
         *
         *  5 3
         * 6 0 1
         *  4 2
         *
         * 0,1,3
         * 0,2,1
         * 0,3,5
         * 0,4,2
         * 0,5,6
         * 0,6,4
         */

        indices.push(vec![0, 1, 3]);
        indices.push(vec![0, 2, 1]);
        for i in 1..(n_segments/2)-1 {
            indices.push(vec![0, 2*i+1, 2*i+3]);
            indices.push(vec![0, 2*i+2, 2*i]);
        }
        indices.push(vec![0, n_segments-1, n_segments]);
        indices.push(vec![0, n_segments, n_segments-2]);

        Ellipse {
            points,
            indices,
        }
    }
}

impl Shape for Ellipse {
    fn points(&self) -> Vec<Point> {
        self.points.clone()
    }
    fn indices(&self) -> Vec<Vec<u32>> {
        self.indices.clone()
    }
    fn vertex_shader(&self) -> String {
        String::from("")
    }
    fn fragment_shader(&self) -> String {
        String::from("")
    }
    fn draw(&self) {
        shape::draw(self);
    }
    fn is_stroke(&self) -> bool {
        false
    }
}
