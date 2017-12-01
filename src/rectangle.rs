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
use sketch::get_stroke_weight;
use transformation::getTransformations;

use na::{Point3, Rotation3, Transform3, Vector3};

use std::f32;

pub fn rect(top_left: Point3<f32>, bottom_right: Point3<f32>) {
    let transformations = getTransformations();
    Rectangle::new(top_left, bottom_right, false, false, transformations).draw();
}

pub struct Rectangle {
    vertex_data: [f32; 9 * 4],
    index_data: [u32; 4],
    is_stroke: bool,
}

pub fn get_rect_vertex_data(
    top_left: Point3<f32>,
    bottom_right: Point3<f32>,
    is_line: bool,
    transformations: Transform3<f32>,
    vertex_data: &mut [f32],
) {
    let mut top_left = top_left;
    let mut bottom_right = bottom_right;
    let top_right;
    let bottom_left;
    if is_line {
        // FIXME: Only works in 2D - need z = 0 to define a plane with the two points
        let width = get_stroke_weight();
        let line = (bottom_right - top_left).normalize() * (width as f32 * 0.5).ceil();
        let anticlockwise =
            Rotation3::from_axis_angle(&Vector3::z_axis(), f32::consts::FRAC_PI_2) * line;
        let clockwise =
            Rotation3::from_axis_angle(&Vector3::z_axis(), -f32::consts::FRAC_PI_2) * line;

        bottom_left = top_left + clockwise;
        top_right = bottom_right + anticlockwise;
        top_left = top_left + anticlockwise;
        bottom_right = bottom_right + clockwise;
    } else {
        bottom_left = Point3::new(top_left.x, bottom_right.y, bottom_right.z);
        top_right = Point3::new(bottom_right.x, top_left.y, top_left.z);
    }

    let color;
    if is_line {
        color = get_stroke();
    } else {
        color = get_fill();
    }

    let transform = glapp::get_transform() * transformations;

    assign_vertex(
        &(transform * top_left),
        &[0.0, 1.0],
        &color,
        &mut vertex_data[0 * 9..],
    );
    assign_vertex(
        &(transform * bottom_left),
        &[0.0, 0.0],
        &color,
        &mut vertex_data[1 * 9..],
    );
    assign_vertex(
        &(transform * top_right),
        &[1.0, 1.0],
        &color,
        &mut vertex_data[2 * 9..],
    );
    assign_vertex(
        &(transform * bottom_right),
        &[1.0, 0.0],
        &color,
        &mut vertex_data[3 * 9..],
    );
}

impl Rectangle {
    pub fn new(
        top_left: Point3<f32>,
        bottom_right: Point3<f32>,
        is_stroke: bool,
        is_line: bool,
        transformations: Transform3<f32>,
    ) -> Rectangle {
        let mut rectangle = Rectangle {
            vertex_data: [0.0; 9 * 4],
            index_data: [0, 1, 2, 3],
            is_stroke,
        };
        get_rect_vertex_data(
            top_left,
            bottom_right,
            is_line,
            transformations,
            &mut rectangle.vertex_data,
        );
        rectangle
    }
}

impl Shape for Rectangle {
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
