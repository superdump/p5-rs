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

use matrix::Matrix;
use point::*;
use shape;
use shape::Shape;
use sketch::get_stroke_weight;
use transformation::getTransformations;

pub fn rect<P: Into<Point>>(top_left: P, bottom_right: P) {
    let transformations = getTransformations();
    Rectangle::new(
        top_left.into(),
        bottom_right.into(),
        false,
        false,
        transformations,
    ).draw();
}

pub struct Rectangle {
    points: Vec<Point>,
    is_stroke: bool,
}

pub fn get_rect_points(
    top_left: Point,
    bottom_right: Point,
    is_line: bool,
    transformations: Matrix,
) -> Vec<Point> {
    let mut top_left = top_left;
    let mut bottom_right = bottom_right;
    let top_right;
    let bottom_left;
    if is_line {
        // FIXME: Only works in 2D - need z = 0 to define a plane with the two points
        let width = get_stroke_weight();
        let half_width = (width as f32 * 0.5).ceil();
        let mut anticlockwise: Point = (
            -(bottom_right.y - top_left.y),
            bottom_right.x - top_left.x,
            0.0,
        ).into();
        anticlockwise.setMag(half_width);
        let mut clockwise: Point = (
            bottom_right.y - top_left.y,
            -(bottom_right.x - top_left.x),
            0.0,
        ).into();
        clockwise.setMag(half_width);
        bottom_left = top_left + clockwise;
        top_right = bottom_right + anticlockwise;
        top_left = top_left + anticlockwise;
        bottom_right = bottom_right + clockwise;
    } else {
        bottom_left = Point {
            x: top_left.x,
            y: bottom_right.y,
            z: bottom_right.z,
        };
        top_right = Point {
            x: bottom_right.x,
            y: top_left.y,
            z: top_left.z,
        };
    }
    let mut points = vec![top_left, bottom_left, top_right, bottom_right];
    for ref mut point in &mut points {
        transformations.transform(point);
    }
    points
}

pub fn get_rect_uvs() -> Vec<f32> {
    vec![0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0]
}

impl Rectangle {
    pub fn new(
        top_left: Point,
        bottom_right: Point,
        is_stroke: bool,
        is_line: bool,
        transformations: Matrix,
    ) -> Rectangle {
        let points = get_rect_points(
            top_left,
            bottom_right,
            is_line,
            transformations,
        );
        Rectangle {
            points,
            is_stroke,
        }
    }
}

impl Shape for Rectangle {
    fn points(&self) -> Vec<Point> {
        self.points.clone()
    }
    fn uvs(&self) -> Vec<f32> {
        get_rect_uvs()
    }
    fn indices(&self) -> Vec<u32> {
        vec![0, 1, 2, 3]
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
        self.is_stroke
    }
}
