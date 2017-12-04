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
use transformation::get_transformations;
use utils::*;

use na::Point3;

pub fn triangle(p1: Point3<f32>, p2: Point3<f32>, p3: Point3<f32>) {
    Triangle::new(p1, p2, p3).draw();
}

pub struct Triangle {
    vertex_data: [f32; 9 * 3],
    index_data: [u32; 3],
}

impl Triangle {
    pub fn new(p1: Point3<f32>, p2: Point3<f32>, p3: Point3<f32>) -> Triangle {
        let mut triangle = Triangle {
            vertex_data: [0.0; 9 * 3],
            index_data: [0, 1, 2],
        };

        let points;
        if have_anticlockwise_winding(&p1, &p2, &p3) {
            points = [p1, p2, p3];
        } else {
            points = [p2, p1, p3];
        }
        let (l, t, r, b) = bounding_box(&points);
        let mut uvs: [f32; 6] = [0.0; 6];

        for i in 0..points.len() {
            uvs[i * 2] = map_f32(points[i].x, l, r, 0.0, 1.0);
            uvs[i * 2 + 1] = map_f32(points[i].y, b, t, 0.0, 1.0);
        }

        let sketch = sketch::get_sketch();
        let mut transform = sketch.transformation;
        {
            let transformations = get_transformations();
            if let Some(transformation) = transformations.last() {
                transform *= transformation;
            }
        }
        let color = &sketch.fill;
        for i in 0..points.len() {
            assign_vertex(
                &(transform * points[i]),
                &uvs[i * 2..],
                color,
                &mut triangle.vertex_data[i * 9..],
            );
        }

        triangle
    }
}

impl Shape for Triangle {
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
        false
    }
}
