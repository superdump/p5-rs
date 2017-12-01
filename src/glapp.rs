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

use channel;
use color::*;
use sketch::SKETCH;
use shader::*;

use self::glutin::GlContext;
use gl;
use gl::types::*;
use na::{Matrix4, Transform3, Vector3};

use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::size_of;
use std::os::raw::c_void;
use std::process::exit;
use std::ptr;
use std::sync::{mpsc, Mutex};

#[derive(Debug)]
pub struct GLShape {
    pub shader_program: GLuint,
    pub index_byte_offset: GLuint,
    pub n_triangles: u32,
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
    static ref DEFAULT_SHADER_PROGRAM: Mutex<GLuint> = Mutex::new(0);
    static ref SHADERS: Mutex<HashMap<String, GLuint>> = Mutex::new(HashMap::new());
    static ref GL_SHAPES: Mutex<Vec<GLShape>> = Mutex::new(Vec::new());
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
    let shader_program: GLuint = *DEFAULT_SHADER_PROGRAM.lock().unwrap();
    if shader_program > 0 {
        return shader_program;
    }

    let (tx, rx) = mpsc::channel::<GLuint>();
    channel::push(Box::new(move || {
        GLAPP.with(|handle| {
            let mut program: GLuint = 0;
            if let Some(ref glapp) = *handle.borrow() {
                program = glapp.default_shader_program;
            }
            tx.send(program).unwrap();
        });
    }));
    channel::send();
    let mut default_shader_program = DEFAULT_SHADER_PROGRAM.lock().unwrap();
    *default_shader_program = rx.recv().unwrap();
    *default_shader_program
}

pub fn get_shader_program(
    vertex_shader_src: Option<String>,
    fragment_shader_src: Option<String>,
) -> GLuint {
    let mut shader_program = get_default_shader_program();

    if vertex_shader_src != None || fragment_shader_src != None {
        let vertex_shader_src = vertex_shader_src.unwrap_or(String::from(DEFAULT_VERTEX_SHADER));
        let fragment_shader_src =
            fragment_shader_src.unwrap_or(String::from(DEFAULT_FRAGMENT_SHADER));

        let concat = format!("{}{}", vertex_shader_src, fragment_shader_src);
        if let Some(program) = SHADERS.lock().unwrap().get(&concat) {
            return *program;
        }

        let (tx, rx) = mpsc::channel::<GLuint>();
        channel::push(Box::new(move || {
            let vertex_shader = compile_shader(&vertex_shader_src, gl::VERTEX_SHADER);
            let fragment_shader = compile_shader(&fragment_shader_src, gl::FRAGMENT_SHADER);
            let program = link_program(vertex_shader, fragment_shader);

            unsafe {
                gl::DeleteShader(vertex_shader);
                gl::DeleteShader(fragment_shader);
            }
            tx.send(program).unwrap();
        }));
        channel::send();
        shader_program = rx.recv().unwrap();
        SHADERS.lock().unwrap().insert(concat, shader_program);
    }
    shader_program
}

pub fn background(color: &Color) {
    unsafe {
        gl::ClearColor(color.x, color.y, color.z, color.w);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn size(w: u32, h: u32) {
    {
        let mut sketch = SKETCH.lock().unwrap();
        sketch.width = w.clone();
        sketch.height = h.clone();
        sketch.transformation =
            Transform3::from_matrix_unchecked(Matrix4::new_nonuniform_scaling(&Vector3::new(
                2.0 / w as f32,
                2.0 / h as f32,
                2.0 / h as f32,
            )));
    }
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

pub fn get_transform() -> Transform3<f32> {
    SKETCH.lock().unwrap().transformation.clone()
}

const N_BUFFERS: usize = 1;

struct GLApp {
    events_loop: glutin::EventsLoop,
    gl_window: glutin::GlWindow,
    default_shader_program: GLuint,
    vaos: [GLuint; N_BUFFERS],
    vbos: [GLuint; N_BUFFERS],
    ebos: [GLuint; N_BUFFERS],
    n_indices: [usize; N_BUFFERS],
    object_index: usize,
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
            default_shader_program: 0,
            vaos: [0; N_BUFFERS],
            vbos: [0; N_BUFFERS],
            ebos: [0; N_BUFFERS],
            n_indices: [0; N_BUFFERS],
            object_index: 0,
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

        self.init_gl_objects();
    }

    fn init_gl_objects(&mut self) {
        unsafe {
            gl::GenVertexArrays(N_BUFFERS as GLsizei, self.vaos.as_mut_ptr());
            gl::GenBuffers(N_BUFFERS as GLsizei, self.vbos.as_mut_ptr());
            gl::GenBuffers(N_BUFFERS as GLsizei, self.ebos.as_mut_ptr());
            for i in 0..N_BUFFERS {
                let vao = self.vaos[i];
                let vbo = self.vbos[i];
                let ebo = self.ebos[i];

                gl::BindVertexArray(vao);
                gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

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

                let uv_attr: GLuint = 1;
                gl::VertexAttribPointer(
                    uv_attr,
                    2,
                    gl::FLOAT,
                    gl::FALSE as GLboolean,
                    (VBO_STRIDE_N * size_of::<GLfloat>()) as GLint,
                    (3 * size_of::<GLfloat>()) as *const c_void,
                );
                gl::EnableVertexAttribArray(uv_attr);

                let col_attr: GLuint = 2;
                gl::VertexAttribPointer(
                    col_attr,
                    4,
                    gl::FLOAT,
                    gl::FALSE as GLboolean,
                    (VBO_STRIDE_N * size_of::<GLfloat>()) as GLint,
                    (5 * size_of::<GLfloat>()) as *const c_void,
                );
                gl::EnableVertexAttribArray(col_attr);
            }
        }
    }

    fn get_next_index(&self) -> usize {
        (self.object_index + 1) % N_BUFFERS
    }

    pub fn upload_data(&mut self, vertex_data: &Vec<GLfloat>, index_data: &Vec<GLuint>) {
        let next_index = self.get_next_index();
        let vbo = self.vaos[next_index];
        let ebo = self.ebos[next_index];
        self.n_indices[next_index] = index_data.len();

        unsafe {
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertex_data.len() * size_of::<GLfloat>()) as GLsizeiptr,
                vertex_data.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (index_data.len() * size_of::<GLuint>()) as GLsizeiptr,
                index_data.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn get_current_objects(&mut self) -> (GLuint, GLuint, GLuint, usize) {
        let current_index = self.object_index;
        self.object_index = self.get_next_index();
        (
            self.vaos[current_index],
            self.vbos[current_index],
            self.ebos[current_index],
            self.n_indices[current_index],
        )
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
pub fn append_data(vertex_data: &[f32], index_data: &[u32]) {
    let total_vertices_before;
    {
        let mut vertices = VERTICES.lock().unwrap();
        total_vertices_before = vertices.len() / VBO_STRIDE_N;
        vertices.extend_from_slice(vertex_data);
    }
    let mut indices = INDICES.lock().unwrap();
    if indices.is_empty() {
        indices.reserve(index_data.len());
    } else {
        indices.reserve(index_data.len() + 2);
        let mut repeated = 0;
        if let Some(last) = indices.last() {
            repeated = (*last).clone();
        }
        indices.push(repeated);
        indices.push(total_vertices_before as u32 + index_data[0]);
    }
    for index in index_data {
        indices.push(total_vertices_before as u32 + index);
    }
}

fn drain_data() -> (Vec<GLfloat>, Vec<GLuint>) {
    let vertex_data;
    let index_data;
    {
        let mut vertices = VERTICES.lock().unwrap();
        vertex_data = vertices.drain(..).collect();
    }
    {
        let mut indices = INDICES.lock().unwrap();
        index_data = indices.drain(..).collect();
    }
    (vertex_data, index_data)
}

pub fn append_shape(shader_program: GLuint, n_triangles: u32) {
    let index_byte_offset = *INDEX_BYTES_OFFSET.lock().unwrap();
    let mut gl_shapes = GL_SHAPES.lock().unwrap();
    gl_shapes.push(GLShape {
        shader_program,
        index_byte_offset,
        n_triangles,
    });
    *INDEX_BYTES_OFFSET.lock().unwrap() +=
        ((n_triangles + 2) * size_of::<GLuint>() as u32) as GLuint;
}

fn drain_shapes() -> Vec<GLShape> {
    let mut gl_shapes = GL_SHAPES.lock().unwrap();
    let drained = gl_shapes.drain(..).collect();
    drained
}

fn drain() -> ((Vec<GLfloat>, Vec<u32>), Vec<GLShape>) {
    let tuple = (drain_data(), drain_shapes());
    *INDEX_BYTES_OFFSET.lock().unwrap() = 0;
    tuple
}

pub fn render() {
    let ((vertex_data, index_data), _) = drain();
    channel::push(Box::new(move || {
        // prepare next frame and get objects for this frame
        let mut objects = (0, 0, 0, 0);
        GLAPP.with(|handle| {
            if let Some(ref mut glapp) = *handle.borrow_mut() {
                glapp.upload_data(&vertex_data, &index_data);
                objects = glapp.get_current_objects();
            }
        });

        let n_indices = objects.3;
        if n_indices == 0 {
            return;
        }

        // draw
        let vao = objects.0;
        let default_shader_program = get_default_shader_program_gl();
        unsafe {
            gl::UseProgram(default_shader_program);
            gl::BindVertexArray(vao);
            gl::DrawElements(
                gl::TRIANGLE_STRIP,
                n_indices as GLsizei,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }

        // cleanup
        unsafe {
            let mut shaders = SHADERS.lock().unwrap();
            for (_, shader) in shaders.drain().take(1) {
                gl::DeleteProgram(shader);
            }
        }
    }));
}
