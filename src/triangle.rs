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

use na::{Transform3, Point3};

pub fn triangle(p1: Point3<f32>, p2: Point3<f32>, p3: Point3<f32>) {
    let transformations = getTransformations();
    Triangle::new(
        p1,
        p2,
        p3,
        transformations,
    ).draw();
}

pub struct Triangle {
    points: Vec<Point3<f32>>,
}

impl Triangle {
    pub fn new(
        p1: Point3<f32>,
        p2: Point3<f32>,
        p3: Point3<f32>,
        transformations: Transform3<f32>,
    ) -> Triangle {
        let points;
        if have_anticlockwise_winding(&p1, &p2, &p3) {
            points = vec![
                transformations * p1,
                transformations * p2,
                transformations * p3
            ];
        } else {
            points = vec![
                transformations * p2,
                transformations * p1,
                transformations * p3
            ];
        }
        Triangle {
            points,
        }
    }
}

impl Shape for Triangle {
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
        vec![0, 1, 2]
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
