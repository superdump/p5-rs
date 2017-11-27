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

extern crate glutin;
extern crate libc;
extern crate lru_cache;

use channel;
use color::*;
use point::*;
use sketch::SKETCH;
use shader::*;
use shape::{HashableShape, Shape};
use utils::map_f32;

use self::glutin::GlContext;
use gl;
use gl::types::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::size_of;
use std::os::raw::c_void;
use std::process::exit;
use std::ptr;
use std::sync::{mpsc, Mutex};

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct GLShape {
    pub n_indices: usize,
    pub vao: GLuint,
    pub vbo: GLuint,
    pub ebo: GLuint,
    pub shader_program: GLuint,
}

impl Drop for GLShape {
    fn drop(&mut self) {
        unsafe {
            if self.shader_program != get_default_shader_program_gl() {
                gl::DeleteProgram(self.shader_program);
            }
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

pub const DEFAULT_WIDTH: u32 = 640;
pub const DEFAULT_HEIGHT: u32 = 360;

/* NOTE: GLAPP is thread-local because:
 * `*mut objc::runtime::Object` cannot be sent between threads safely
 * It must be used only from the main thread as macOS requires GL windows are
 * created from the main thread.
 */
thread_local! {
    static GLAPP: RefCell<Option<GLApp>> = RefCell::new(None);
}

lazy_static! {
    static ref VERTICES: Mutex<Vec<GLfloat>> = Mutex::new(Vec::new());
    static ref INDICES: Mutex<Vec<GLuint>> = Mutex::new(Vec::new());
    static ref SHADERS: Mutex<HashMap<String, GLuint>> = Mutex::new(HashMap::new());
    static ref GL_SHAPES: Mutex<lru_cache::LruCache<HashableShape, GLShape>> = Mutex::new(lru_cache::LruCache::new(1_000_000));
    static ref INDEX_BYTES_OFFSET: Mutex<u32> = Mutex::new(0);
}

pub fn listen(rx: mpsc::Receiver<channel::MessageType>) {
    for closures in rx {
        for boxed_closure in closures {
            boxed_closure();
        }
    }
}

pub fn setup() {
    GLAPP.with(|handle| {
        handle.replace(Some(GLApp::new(0, 0)));
    });
    channel::push(Box::new(move || {
        GLAPP.with(|handle| {
            if let Some(ref mut glapp) = *handle.borrow_mut() {
                println!("Running GL setup for the window");
                glapp.setup();
            }
        });
    }));
    channel::send();
}

pub fn poll_events() {
    channel::push(Box::new(move || {
        GLAPP.with(|handle| {
            if let Some(ref mut glapp) = *handle.borrow_mut() {
                glapp.poll_events();
            }
        });
    }));
}

pub fn swap_buffers() {
    channel::push(Box::new(move || {
        GLAPP.with(|handle| {
            if let Some(ref mut glapp) = *handle.borrow_mut() {
                glapp.swap_buffers();
            }
        });
    }));
}

fn get_default_shader_program_gl() -> GLuint {
    let mut program: GLuint = 0;
    GLAPP.with(|handle| {
        if let Some(ref glapp) = *handle.borrow() {
            program = glapp.default_shader_program;
        }
    });
    program
}

pub fn get_default_shader_program() -> GLuint {
    let (tx, rx) = mpsc::channel::<GLuint>();
    channel::push(Box::new(move || {
        GLAPP.with(|handle| {
            let mut default_shader_program: GLuint = 0;
            if let Some(ref glapp) = *handle.borrow() {
                default_shader_program = glapp.default_shader_program;
            }
            tx.send(default_shader_program).unwrap();
        });
    }));
    channel::send();
    let p = rx.recv().unwrap();
    p
}

pub fn get_shader_program(vertex_shader_src: String, fragment_shader_src: String) -> GLuint {
    let mut vertex_shader_src = vertex_shader_src;
    let mut fragment_shader_src = fragment_shader_src;
    let mut shader_program = get_default_shader_program();

    let concat = format!("{}{}", vertex_shader_src, fragment_shader_src);
    if let Some(shader_program) = SHADERS.lock().unwrap().get(&concat) {
        return *shader_program
    }

    if vertex_shader_src.len() > 0 || fragment_shader_src.len() > 0 {
        if vertex_shader_src.len() == 0 {
            vertex_shader_src = String::from(DEFAULT_VERTEX_SHADER);
        } else if fragment_shader_src.len() == 0 {
            fragment_shader_src = String::from(DEFAULT_FRAGMENT_SHADER);
        }

        let (tx, rx) = mpsc::channel::<GLuint>();
        channel::push(Box::new(move || {
            let vertex_shader = compile_shader(&vertex_shader_src, gl::VERTEX_SHADER);
            let fragment_shader = compile_shader(&fragment_shader_src, gl::FRAGMENT_SHADER);
            let shader_program = link_program(vertex_shader, fragment_shader);

            unsafe {
                gl::DeleteShader(vertex_shader);
                gl::DeleteShader(fragment_shader);
            }
            tx.send(shader_program).unwrap();
        }));
        channel::send();
        shader_program = rx.recv().unwrap();
    }
    SHADERS.lock().unwrap().insert(concat, shader_program);
    shader_program
}

pub fn background(color: &Color) {
    let &Color { r, g, b, a } = color;
    unsafe {
        gl::ClearColor(r, g, b, a);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn size(w: u32, h: u32) {
    SKETCH.lock().unwrap().width = w.clone();
    SKETCH.lock().unwrap().height = h.clone();
    channel::push(Box::new(move || {
        GLAPP.with(|handle| {
            if let Some(ref mut glapp) = *handle.borrow_mut() {
                println!("Setting size to {}x{}", w, h);
                glapp.size(w, h);
            }
        });
    }));
    channel::send();
}

struct GLApp {
    events_loop: glutin::EventsLoop,
    gl_window: glutin::GlWindow,
    default_shader_program: GLuint,
}

impl GLApp {
    pub fn new(w: u32, h: u32) -> GLApp {
        let mut w = w;
        let mut h = h;
        if w == 0 {
            w = DEFAULT_WIDTH;
        }
        if h == 0 {
            h = DEFAULT_HEIGHT;
        }
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title("p5-rs sketch")
            .with_dimensions(w, h);
        let context = glutin::ContextBuilder::new().with_vsync(true);
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        GLApp {
            events_loop,
            gl_window,
            default_shader_program: 123456, // FIXME: Is there any good value to use here?
        }
    }

    pub fn setup(&mut self) {
        unsafe {
            self.gl_window.make_current().unwrap();
        }

        gl::load_with(|symbol| self.gl_window.get_proc_address(symbol) as *const _);
        unsafe {
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);
        }

        let vertex_shader = compile_shader(&DEFAULT_VERTEX_SHADER, gl::VERTEX_SHADER);
        let fragment_shader = compile_shader(&DEFAULT_FRAGMENT_SHADER, gl::FRAGMENT_SHADER);
        self.default_shader_program = link_program(vertex_shader, fragment_shader);
    }

    pub fn size(&mut self, w: u32, h: u32) {
        if w == 0 || h == 0 {
            return;
        }
        self.gl_window.window().set_inner_size(w, h);
        self.gl_window.resize(w, h);
    }

    pub fn poll_events(&mut self) {
        let gl_window = &self.gl_window;
        self.events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => exit(0),
                glutin::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(glutin::VirtualKeyCode::Escape) = input.virtual_keycode {
                        exit(0);
                    }
                }
                glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                _ => (),
            },
            _ => (),
        });
    }

    pub fn swap_buffers(&mut self) {
        self.gl_window.swap_buffers().unwrap();
    }
}

// vertices are stored as:
// vertex, color
// Khronos advise to use 4-byte alignment for vertex attributes
// vertex is xyz as 3 GLfloat (12 bytes)
// uv is uv as 2 GLfloat (8 bytes)
// color is rgba as 4 GLfloat (16 bytes)
// the stride is therefore 9 GLfloat (36 bytes)
const VBO_STRIDE_N: usize = 9;
fn create_objects(vertex_data: &Vec<GLfloat>, index_data: &Vec<GLuint>) -> (GLuint, GLuint, GLuint) {
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
            (vertex_data.len() * size_of::<GLfloat>()) as GLsizeiptr,
            vertex_data.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (index_data.len() * size_of::<GLuint>()) as GLsizeiptr,
            index_data.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        // Specify the layout of the vertex data
        let pos_attr: GLuint = 0;
        gl::VertexAttribPointer(
            pos_attr,
            3,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            (VBO_STRIDE_N * size_of::<GLfloat>()) as GLint,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(pos_attr);

        // Specify the color
        let col_attr: GLuint = 1;
        gl::VertexAttribPointer(
            col_attr,
            4,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            (VBO_STRIDE_N * size_of::<GLfloat>()) as GLint,
            (5 * size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(col_attr);

        // Specify the UVs
        let uv_attr: GLuint = 2;
        gl::VertexAttribPointer(
            uv_attr,
            2,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            (VBO_STRIDE_N * size_of::<GLfloat>()) as GLint,
            (3 * size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(uv_attr);
    }

    (vao, vbo, ebo)
}

fn point_to_vertex(point: &Point) -> [GLfloat; 3] {
    let sketch = SKETCH.lock().unwrap();
    let max_w = (sketch.width/2) as f32;
    let min_w = -max_w;
    let max_h = (sketch.height/2) as f32;
    let min_h = -max_h;
    [
        map_f32(point.x.into(), min_w, max_w, -1.0, 1.0),
        map_f32(point.y.into(), min_h, max_h, -1.0, 1.0),
        map_f32(point.z.into(), min_h, max_h, -1.0, 1.0), // FIXME: think about how to convert z
    ]
}

pub fn points_to_vertices(points: &Vec<Point>) -> Vec<GLfloat> {
    let mut vertices = Vec::new();
    for point in points {
        let vertex = point_to_vertex(point);
        vertices.extend_from_slice(&vertex);
    }
    vertices
}

fn make_vertex_data(points: &Vec<Point>, uvs: &Vec<f32>, color: &Color) -> Vec<GLfloat> {
    let vertices = points_to_vertices(points);
    let mut vertex_data = Vec::new();
    let count = vertices.len() / 3;
    for i in 0..count {
        let vd_offset = i * 3;
        vertex_data.extend_from_slice(&vertices[vd_offset..vd_offset+3]);
        let uv_offset = i * 2;
        vertex_data.extend_from_slice(&uvs[uv_offset..uv_offset+2]);
        vertex_data.extend_from_slice(color.as_vec4().as_slice());
    }
    vertex_data
}

fn draw(vao: GLuint, shader_program: GLuint, n_indices: usize) {
    unsafe {
        gl::BindVertexArray(vao);
        gl::UseProgram(shader_program);
        gl::DrawElements(
            gl::TRIANGLE_STRIP,
            n_indices as GLsizei,
            gl::UNSIGNED_INT,
            ptr::null(),
        );
    }
}

pub fn draw_hashable_shape(hashable: HashableShape, shape: &Shape) {
    let contains_key = GL_SHAPES.lock().unwrap().contains_key(&hashable);
    if contains_key {
        let mut vao: GLuint = 0;
        let mut shader_program: GLuint = 0;
        let mut n_indices: usize = 0;
        if let Some(gl_shape) = GL_SHAPES.lock().unwrap().get_mut(&hashable) {
            vao = gl_shape.vao;
            shader_program = gl_shape.shader_program;
            n_indices = gl_shape.n_indices;
        }
        channel::push(Box::new(move || {
            draw(vao, shader_program, n_indices);
        }));
        channel::send();
        return;
    }

    let points = shape.points();
    let uvs = shape.uvs();
    let color: Color = hashable.color.into();

    let vertex_data = make_vertex_data(&points, &uvs, &color);
    let index_data = shape.indices();
    let shader_program = get_shader_program(shape.vertex_shader(), shape.fragment_shader());

    let (tx, rx) = mpsc::channel::<GLShape>();
    channel::push(Box::new(move || {
        let (vao, vbo, ebo) = create_objects(&vertex_data, &index_data);
        tx.send(GLShape {
            n_indices: index_data.len(),
            vao,
            vbo,
            ebo,
            shader_program,
        }).unwrap();
        draw(vao, shader_program, index_data.len());
    }));
    channel::send();
    let gl_shape = rx.recv().unwrap();
    GL_SHAPES.lock().unwrap().insert(hashable, gl_shape);
}