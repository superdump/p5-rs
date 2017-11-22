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
use point::*;
use sketch::SKETCH;
use utils::*;

use gl::types::*;

#[derive(Debug)]
pub struct GLShape {
    pub shader_program: GLuint,
    pub index_byte_offset: GLuint,
    pub n_triangles: u32,
}

pub trait Shape {
    fn points(&self) -> Vec<Point>;
    fn indices(&self) -> Vec<Vec<u32>>;
    fn vertex_shader(&self) -> String;
    fn fragment_shader(&self) -> String;
    fn draw(&self);
    fn is_stroke(&self) -> bool;
}

fn point_to_vertex(point: Point) -> [GLfloat; 3] {
    let sketch = SKETCH.lock().unwrap();
    let max_w = (sketch.width/2) as f64;
    let min_w = -max_w;
    let max_h = (sketch.height/2) as f64;
    let min_h = -max_h;
    [
        map(point.x as f64, min_w, max_w, -1.0, 1.0) as GLfloat,
        map(point.y as f64, min_h, max_h, -1.0, 1.0) as GLfloat,
        map(point.z as f64, min_h, max_h, -1.0, 1.0) as GLfloat, // FIXME: think about how to convert z
    ]
}

pub fn points_to_vertices(points: Vec<Point>) -> Vec<GLfloat> {
    let mut vertices = Vec::new();
    for point in points {
        let vertex = point_to_vertex(point);
        vertices.extend_from_slice(&vertex);
    }
    vertices
}

pub fn draw(shape: &Shape) {
    let vertices = shape.points();
    let vertex_data = points_to_vertices(vertices);
    let index_data = shape.indices();
    let color;
    if shape.is_stroke() {
        color = get_stroke().as_vec4();
    } else {
        color = get_fill().as_vec4();
    }

    let n_triangles = index_data.len();
    let total_vertices_before = append_vertices(&vertex_data, &color);
    for indices in index_data {
        append_indices(total_vertices_before, indices);
    }
    let shader_program = get_shader_program(
        shape.vertex_shader(),
        shape.fragment_shader(),
    );
    append_shape(shader_program, n_triangles as u32);
}
