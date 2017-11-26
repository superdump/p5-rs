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

use gl::types::*;

#[derive(Debug)]
pub struct GLShape {
    pub shader_program: GLuint,
    pub index_byte_offset: GLuint,
    pub n_triangles: u32,
}

pub trait Shape {
    fn points(&self) -> Vec<Point>;
    fn uvs(&self) -> Vec<f32>;
    fn indices(&self) -> Vec<u32>;
    fn vertex_shader(&self) -> String;
    fn fragment_shader(&self) -> String;
    fn draw(&self);
    fn is_stroke(&self) -> bool;
}

pub fn draw(shape: &Shape) {
    let vertices = shape.points();
    let uvs = shape.uvs();
    let color;
    if shape.is_stroke() {
        color = get_stroke();
    } else {
        color = get_fill();
    }

    let indices = shape.indices();
    let n_triangles = indices.len() - 2;
    let total_vertices_before = append_vertices(&vertices, &uvs, &color);
    append_indices(total_vertices_before, indices);
    let shader_program = get_shader_program(
        shape.vertex_shader(),
        shape.fragment_shader(),
    );
    append_shape(shader_program, n_triangles as u32);
}
