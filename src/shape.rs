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

use color::Color;
use glapp::*;

use na::Point3;

pub trait Shape {
    fn vertex_data(&self) -> &[f32];
    fn index_data(&self) -> &[u32];
    fn vertex_shader(&self) -> Option<String>;
    fn fragment_shader(&self) -> Option<String>;
    fn draw(&self);
    fn is_stroke(&self) -> bool;
}

pub fn assign_vertex(p: &Point3<f32>, uv: &[f32], c: &Color, vd: &mut [f32]) {
    vd[0] = p.x;
    vd[1] = p.y;
    vd[2] = p.z;
    vd[3] = uv[0];
    vd[4] = uv[1];
    vd[5] = c.x;
    vd[6] = c.y;
    vd[7] = c.z;
    vd[8] = c.w;
}

pub fn draw(shape: &Shape) {
    let vertex_data = shape.vertex_data();
    let index_data = shape.index_data();
    let n_triangles = index_data.len() - 2;
    let total_vertices_before = append_data(vertex_data, index_data);
    let shader_program = get_shader_program(shape.vertex_shader(), shape.fragment_shader());
    append_shape(shader_program, n_triangles as u32);
}
