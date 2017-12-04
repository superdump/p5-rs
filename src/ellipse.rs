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
use shape::*;
use sketch;
use transformation::getTransformations;

use na::{Point3, Translation, Vector3};

use std::f32::consts::*;

const N_SEGMENTS: u32 = 64;

pub fn ellipse(center: Point3<f32>, width: f32, height: f32) {
    Ellipse::new(center.into(), width, height, N_SEGMENTS, false).draw();
}

pub struct Ellipse {
    vertex_data: [f32; 9 * N_SEGMENTS as usize],
    index_data: [u32; N_SEGMENTS as usize],
    n_segments: usize,
    is_stroke: bool,
}

impl Ellipse {
    pub fn new(
        center: Point3<f32>,
        width: f32,
        height: f32,
        n_segments: u32,
        is_stroke: bool,
    ) -> Ellipse {
        let n_segments = n_segments as usize;
        let mut ellipse = Ellipse {
            vertex_data: [0.0; 9 * N_SEGMENTS as usize],
            index_data: [0; N_SEGMENTS as usize],
            n_segments,
            is_stroke,
        };

        let a = width * 0.5;
        let b = height * 0.5;

        let point_at_angle =
            |angle: f32| -> Point3<f32> { Point3::new(a * angle.sin(), b * angle.cos(), 0.0) };

        let sketch = sketch::get_sketch();
        let color;
        if is_stroke {
            color = &sketch.stroke;
        } else {
            color = &sketch.fill;
        }

        let transform;
        {
            let transformations = getTransformations();
            if let Some(transformation) = transformations.last() {
                transform = sketch.transformation * transformation
                    * Translation::from_vector(center - Point3::origin());
            } else {
                transform =
                    sketch.transformation * Translation::from_vector(center - Point3::origin());
            }
        }

        assign_vertex(
            &(transform * Point3::new(-a, 0.0, 0.0)),
            &[0.0, 0.5],
            color,
            &mut ellipse.vertex_data[0 * 9..],
        );
        let last_index = n_segments - 1;
        assign_vertex(
            &(transform * Point3::new(a, 0.0, 0.0)),
            &[1.0, 0.5],
            &color,
            &mut ellipse.vertex_data[last_index * 9..],
        );
        let center_index = last_index / 2;
        let mut n_points_remaining = n_segments - 2;
        if n_segments % 4 == 0 {
            assign_vertex(
                &(transform * Point3::new(0.0, -b, 0.0)),
                &[0.5, 0.0],
                &color,
                &mut ellipse.vertex_data[center_index * 9..],
            );
            assign_vertex(
                &(transform * Point3::new(0.0, b, 0.0)),
                &[0.5, 1.0],
                color,
                &mut ellipse.vertex_data[(center_index + 1) * 9..],
            );
            n_points_remaining -= 2;
        }

        let do_vertex = |p: Point3<f32>, offsets: &Vector3<f32>, vd: &mut [f32]| {
            let point = p + offsets;
            assign_vertex(
                &(transform * point),
                &[0.5 * (point.x / a + 1.0), 0.5 * (point.y / b + 1.0)],
                color,
                vd,
            );
        };

        let da = 2.0 * PI / (n_segments as f32);
        for i in 1..(n_points_remaining / 4) + 1 {
            let p = point_at_angle(-0.5 * PI + (i as f32) * da);
            let offsets = [
                Vector3::new(0.0, -2.0 * p.y, 0.0),        // bl
                Vector3::new(0.0, 0.0, 0.0),               // tl
                Vector3::new(-2.0 * p.x, -2.0 * p.y, 0.0), // br
                Vector3::new(-2.0 * p.x, 0.0, 0.0),        // tr
            ];
            do_vertex(p, &offsets[0], &mut ellipse.vertex_data[(2 * i - 1) * 9..]);
            do_vertex(p, &offsets[1], &mut ellipse.vertex_data[(2 * i) * 9..]);
            do_vertex(
                p,
                &offsets[2],
                &mut ellipse.vertex_data[(last_index - (2 * i)) * 9..],
            );
            do_vertex(
                p,
                &offsets[3],
                &mut ellipse.vertex_data[(last_index - (2 * i - 1)) * 9..],
            );
        }

        for i in 0..n_segments {
            ellipse.index_data[i] = i as u32;
        }

        ellipse
    }
}

impl Shape for Ellipse {
    fn vertex_data(&self) -> &[f32] {
        &self.vertex_data[..self.n_segments * 9]
    }
    fn index_data(&self) -> &[u32] {
        &self.index_data[..self.n_segments]
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
