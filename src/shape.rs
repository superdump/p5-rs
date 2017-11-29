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
use glapp::*;
use matrix::Matrix;
use point::*;

use ordered_float;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct HashableShape {
    pub color: HashableColor,
    pub points: Vec<HashablePoint>,
    pub uvs: Vec<ordered_float::OrderedFloat<f32>>,
    pub indices: Vec<u32>,
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub is_stroke: bool,
}

pub trait Shape {
    fn points(&self) -> Vec<Point>;
    fn transformations(&self) -> Matrix;
    fn uvs(&self) -> Vec<f32>;
    fn indices(&self) -> Vec<u32>;
    fn vertex_shader(&self) -> String;
    fn fragment_shader(&self) -> String;
    fn draw(&self);
    fn is_stroke(&self) -> bool;
}

pub fn draw(shape: &Shape) {
    let is_stroke = shape.is_stroke();
    let color;
    if is_stroke {
        color = get_stroke().as_hashable();
    } else {
        color = get_fill().as_hashable();
    }

    let mut points = shape.points();
    let transformations = shape.transformations();
    for ref mut point in &mut points {
        transformations.transform(point);
    }
    let hashable_points = points.iter().map(|p| p.as_hashable()).collect();

    let uvs_f32 = shape.uvs();
    let mut uvs: Vec<ordered_float::OrderedFloat<f32>> = Vec::new();
    for uv in uvs_f32 {
        uvs.push(ordered_float::OrderedFloat(uv));
    }

    let indices = shape.indices();

    let vertex_shader = shape.vertex_shader();
    let fragment_shader = shape.fragment_shader();

    let hashable = HashableShape {
        color,
        points: hashable_points,
        uvs,
        indices,
        vertex_shader,
        fragment_shader,
        is_stroke,
    };

    draw_hashable_shape(hashable, shape);
}
