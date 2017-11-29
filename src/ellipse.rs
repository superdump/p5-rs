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
use rectangle::{get_rect_points, get_rect_uvs};
use shape;
use shape::Shape;
use transformation::getTransformations;

pub fn ellipse<P: Into<Point>>(center: P, width: f32, height: f32) {
    let transformations = getTransformations();
    Ellipse::new(
        center.into(),
        width,
        height,
        false,
        transformations,
    ).draw();
}

pub struct Ellipse {
    points: Vec<Point>,
    is_stroke: bool,
}

impl Ellipse {
    pub fn new(
        center: Point,
        width: f32,
        height: f32,
        is_stroke: bool,
        transformations: Matrix,
    ) -> Ellipse {
        let diagonal: Point = (-0.5 * width, 0.5 * height).into();
        let top_left: Point = center + diagonal;
        let bottom_right: Point = center - diagonal;
        let points = get_rect_points(
            top_left,
            bottom_right,
            false,
            transformations,
        );
        Ellipse {
            points,
            is_stroke,
        }
    }
}

const VERTEX_SHADER: &'static str = "#version 330 core\n\
    layout (location = 0) in vec3 position;\n\
    layout (location = 1) in vec4 a_color;\n\
    layout (location = 2) in vec2 uv;\n\
    out vec4 color;\n\
    out vec2 tex_coord;\n\
    void main() {\n\
        gl_Position = vec4(position.x, position.y, position.z, 1.0);\n\
        color = a_color;\n\
        tex_coord = uv;\n\
    }";

const FRAGMENT_SHADER: &'static str = "#version 330 core\n\
    in vec4 color;\n\
    in vec2 tex_coord;\n\
    out vec4 frag_color;\n\
    void main() {\n\
        vec2 scaled = tex_coord * 2.0 - 1.0;
        float d = dot(scaled, scaled);\n\
        if (d > 1) {\n\
            discard;\n\
        }\n\
        frag_color = color;\n\
    }";

impl Shape for Ellipse {
    fn points(&self) -> Vec<Point> {
        self.points.clone()
    }
    fn uvs(&self) -> Vec<f32> {
        get_rect_uvs()
    }
    fn indices(&self) -> Vec<u32> {
        (0..4).collect()
    }
    fn vertex_shader(&self) -> String {
        String::from(VERTEX_SHADER)
    }
    fn fragment_shader(&self) -> String {
        String::from(FRAGMENT_SHADER)
    }
    fn draw(&self) {
        shape::draw(self);
    }
    fn is_stroke(&self) -> bool {
        self.is_stroke
    }
}
