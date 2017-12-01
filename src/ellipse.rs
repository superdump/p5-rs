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
use glapp;
use shape;
use shape::*;
use transformation::getTransformations;

use na::{Point3, Transform3, Translation, Vector3};

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
    vertex_data: [f32; 9 * N_SEGMENTS as usize],
    index_data: [u32; 64],
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
        let mut ellipse = Ellipse {
            vertex_data: [0.0; 9 * N_SEGMENTS as usize],
            index_data: [0; N_SEGMENTS as usize],
            is_stroke,
        };

        let a = width * 0.5;
        let b = height * 0.5;

        let point_at_angle =
            |angle: f32| -> Point3<f32> { Point3::new(a * angle.sin(), b * angle.cos(), 0.0) };

        let color;
        if is_stroke {
            color = get_stroke();
        } else {
            color = get_fill();
        }

        let transform = glapp::get_transform() * transformations
            * Translation::from_vector(center - Point3::origin());
        assign_vertex(
            &(transform * point_at_angle(-0.5 * PI)),
            &[0.0, 0.5],
            &color,
            &mut ellipse.vertex_data,
        );
        let da = 2.0 * PI / (N_SEGMENTS as f32);
        for i in 1..N_SEGMENTS / 2 {
            let p = point_at_angle(-0.5 * PI + (i as f32) * da);
            let index = (i as usize * 2 - 1) * 9;
            assign_vertex(
                &(transform * (p + Vector3::new(0.0, -2.0 * p.y, 0.0))),
                &[p.x / a, -p.y / b],
                &color,
                &mut ellipse.vertex_data[index..],
            );
            assign_vertex(
                &(transform * p),
                &[p.x / a, p.y / b],
                &color,
                &mut ellipse.vertex_data[index + 9..],
            );
        }
        assign_vertex(
            &(transform * point_at_angle(0.5 * PI)),
            &[1.0, 0.5],
            &color,
            &mut ellipse.vertex_data[(N_SEGMENTS as usize - 1) * 9..],
        );

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

        for i in 0..64 {
            ellipse.index_data[i] = i as u32;
        }

        ellipse
    }
}

impl Shape for Ellipse {
    fn vertex_data(&self) -> &[f32] {
        &self.vertex_data
    }
    fn index_data(&self) -> &[u32] {
        &self.index_data
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
