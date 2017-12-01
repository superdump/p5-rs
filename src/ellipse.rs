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

use shape;
use shape::Shape;
use transformation::getTransformations;
use utils::*;

use na::{Transform3, Point3, Vector3};

use std::f32::consts::*;

const N_SEGMENTS: u32 = 64;

pub fn ellipse(center: Point3<f32>, width: f32, height: f32) {
    let transformations = getTransformations();
    Ellipse::new(
        center.into(),
        width,
        height,
        N_SEGMENTS,
        false,
        transformations,
    ).draw();
}

pub struct Ellipse {
    points: Vec<Point3<f32>>,
    indices: Vec<u32>,
    is_stroke: bool,
}

impl Ellipse {
    pub fn new(
        center: Point3<f32>,
        width: f32,
        height: f32,
        n_segments: u32,
        is_stroke: bool,
        transformations: Transform3<f32>,
    ) -> Ellipse {
        let a = width * 0.5;
        let b = height * 0.5;

        let offset_from_angle = |angle: f32| -> Vector3<f32> {
            Vector3::new(
                a * angle.sin(),
                b * angle.cos(),
                0.0,
            )
        };

        let mut points: Vec<Point3<f32>> = Vec::with_capacity(n_segments as usize);
        points.push(transformations * (center + offset_from_angle(-0.5 * PI)));
        let da = 2.0 * PI / (n_segments as f32);
        for i in 1..n_segments / 2 {
            let offset = offset_from_angle(-0.5 * PI + (i as f32) * da);
            points.push(
                transformations * (center + Vector3::new(offset.x, -offset.y, offset.z)),
            );
            points.push(transformations * (center + offset));
        }
        points.push(transformations * (center + offset_from_angle(0.5 * PI)));

        /* e.g.
         * n_segments = 6
         * always anti-clockwise through vertices
         *
         *  2 4
         * 0   5
         *  1 3
         *
         * 0,1,2,3,4,5 as a triangle strip
         */

        let indices: Vec<u32> = (0..n_segments).collect();

        Ellipse {
            points,
            indices,
            is_stroke,
        }
    }
}

impl Shape for Ellipse {
    fn points(&self) -> &Vec<Point3<f32>> {
        &self.points
    }
    fn uvs(&self) -> Vec<f32> {
        let (l, t, r, b) = bounding_box(&self.points);
        let mut uvs = Vec::new();
        for point in &self.points {
            uvs.push(map_f32(point.x, l, r, 0.0, 1.0));
            uvs.push(map_f32(point.y, b, t, 0.0, 1.0));
        }
        uvs
    }
    fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }
    fn vertex_shader(&self) -> Option<String> {
        None
    }
    fn fragment_shader(&self) -> Option<String> {
        None
    }
    fn draw(&self) {
        shape::draw(self);
    }
    fn is_stroke(&self) -> bool {
        self.is_stroke
    }
}
