use point::*;
use shader::*;

use gl::types::*;
use gl;

use std::mem;
use std::ptr;
use std::ffi::CString;

pub trait Shape {
    fn points(&self) -> Vec<Point>;
    fn vertex_shader(&self) -> String;
    fn fragment_shader(&self) -> String;
    fn draw(&self);
}

fn point_to_vertex(point: Point) -> [GLfloat; 3] {
    [
        point.x as GLfloat / 640.0 as GLfloat,
        point.y as GLfloat / 360.0 as GLfloat,
        point.z as GLfloat / 360.0 as GLfloat // FIXME: think about how to convert z
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
    let mut index_data = Vec::new();
    for i in 0..vertex_data.len() {
        index_data.push(i as GLuint);
    }

    let vertex_shader_src = shape.vertex_shader();
    let fragment_shader_src = shape.fragment_shader();

    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;
    let vertex_shader = compile_shader(&vertex_shader_src, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(&fragment_shader_src, gl::FRAGMENT_SHADER);
    let shader_program = link_program(vertex_shader, fragment_shader);

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
        let pos_attr = gl::GetAttribLocation(shader_program, CString::new("position").unwrap().as_ptr());
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

    unsafe {
        gl::UseProgram(shader_program);
        gl::BindVertexArray(vao);
        gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, ptr::null());
    }
}
