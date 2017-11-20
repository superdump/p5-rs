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

use channel;
use glapp::get_default_shader_program;
use point::*;
use shader::*;
use sketch::SKETCH;
use utils::*;

use gl::types::*;
use gl;

use std::mem;
use std::ptr;
use std::ffi::CString;

pub trait Shape {
    fn points(&self) -> Vec<Point>;
    fn indices(&self) -> Vec<Vec<u32>>;
    fn vertex_shader(&self) -> String;
    fn fragment_shader(&self) -> String;
    fn draw(&self);
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

fn make_triangle(vertex_data: &Vec<GLfloat>, index_data: &Vec<GLuint>, shader_program: &GLuint) -> (GLuint, GLuint, GLuint) {
    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&vertex_data[0]),
            gl::STATIC_DRAW,
        );

        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (index_data.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
            mem::transmute(&index_data[0]),
            gl::STATIC_DRAW,
        );

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(*shader_program, CString::new("position").unwrap().as_ptr());
        gl::VertexAttribPointer(
            pos_attr as GLuint,
            3,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            3 * mem::size_of::<GLfloat>() as GLint,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(pos_attr as GLuint);
    }

    (vao, vbo, ebo)
}

pub fn draw(shape: &Shape) {
    let vertices = shape.points();
    let vertex_data = points_to_vertices(vertices);
    let index_data = shape.indices();

    let mut vertex_shader_src = shape.vertex_shader();
    let mut fragment_shader_src = shape.fragment_shader();

    channel::send_closure(Box::new(move || {
        // prepare
        let default_shader_program = get_default_shader_program();
        let shader_program: GLuint;
        if vertex_shader_src.len() > 0 || fragment_shader_src.len() > 0 {
            if vertex_shader_src.len() == 0 {
                vertex_shader_src = String::from(DEFAULT_VERTEX_SHADER);
            } else if fragment_shader_src.len() == 0 {
                fragment_shader_src = String::from(DEFAULT_FRAGMENT_SHADER);
            }
            let vertex_shader = compile_shader(&vertex_shader_src, gl::VERTEX_SHADER);
            let fragment_shader = compile_shader(&fragment_shader_src, gl::FRAGMENT_SHADER);
            shader_program = link_program(vertex_shader, fragment_shader);

            unsafe {
                gl::DeleteShader(vertex_shader);
                gl::DeleteShader(fragment_shader);
            }
        } else {
            shader_program = default_shader_program;
        }

        let mut vao: Vec<GLuint> = Vec::new();
        let mut vbo: Vec<GLuint> = Vec::new();
        let mut ebo: Vec<GLuint> = Vec::new();

        for triangle in index_data {
            let (tri_vao, tri_vbo, tri_ebo) = make_triangle(&vertex_data, &triangle, &shader_program);
            vao.push(tri_vao);
            vbo.push(tri_vbo);
            ebo.push(tri_ebo);
        }

        // draw
        unsafe {
            for triangle in &vao {
                gl::UseProgram(shader_program);
                gl::BindVertexArray(*triangle);
                gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, ptr::null());
            }
        }

        // cleanup
        unsafe{
            if shader_program != default_shader_program {
                gl::DeleteProgram(shader_program);
            }
            for buffer in ebo {
                gl::DeleteBuffers(1, &buffer);
            }
            for buffer in vbo {
                gl::DeleteBuffers(1, &buffer);
            }
            for array in vao {
                gl::DeleteVertexArrays(1, &array);
            }
        }
    }));
}
