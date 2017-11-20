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

use std::cell::RefCell;
use std::process::exit;
use std::sync::mpsc;

pub const DEFAULT_WIDTH: u32 = 640;
pub const DEFAULT_HEIGHT: u32 = 360;

thread_local! {
    static GLAPP: RefCell<Option<GLApp>> = RefCell::new(None);
}

pub fn listen(rx: mpsc::Receiver<channel::ClosureType>) {
    for boxed_closure in rx {
        boxed_closure();
    }
}

pub fn setup() {
    GLAPP.with(|handle| {
        handle.replace(Some(GLApp::new(0, 0)));
    });
    channel::send_closure(Box::new(move || {
        GLAPP.with(|handle| {
            if let Some(ref mut glapp) = *handle.borrow_mut() {
                glapp.setup();
            }
        });
    }));
}

pub fn poll_events() {
    channel::send_closure(Box::new(move || {
        GLAPP.with(|handle| {
            if let Some(ref mut glapp) = *handle.borrow_mut() {
                glapp.poll_events();
            }
        });
    }));
}

pub fn swap_buffers() {
    channel::send_closure(Box::new(move || {
        GLAPP.with(|handle| {
            if let Some(ref mut glapp) = *handle.borrow_mut() {
                glapp.swap_buffers();
            }
        });
    }));
}

pub fn get_default_shader_program() -> GLuint {
    let mut program: GLuint = 0;
    GLAPP.with(|handle| {
        if let Some(ref glapp) = *handle.borrow() {
            program = glapp.default_shader_program;
        }
    });
    program
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
    channel::send_closure(Box::new(move || {
        GLAPP.with(|handle| {
            if let Some(ref mut glapp) = *handle.borrow_mut() {
                glapp.size(w, h);
            }
        });
    }));
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
