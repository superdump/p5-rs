extern crate gl;
extern crate glutin;
extern crate libc;

use gl::types::*;
use glutin::GlContext;

pub mod sketch;

use sketch::*;

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub struct App<'a> {
    sketch: &'a mut Sketch,
    width: u32,
    height: u32,
    background: Color,
    events_loop: glutin::EventsLoop,
    gl_window: glutin::GlWindow
}

impl<'a> App<'a> {
    pub fn new(sketch: &'a mut Sketch) -> App<'a> {
        let w = 640;
        let h = 360;

        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title("p5-rs sketch")
            .with_dimensions(w, h);
        let context = glutin::ContextBuilder::new().with_vsync(true);
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        App {
            width: w,
            height: h,
            sketch: sketch,
            background: Color{ r: 0.2, g: 0.2, b: 0.2, a: 1.0 },
            events_loop: events_loop,
            gl_window: gl_window
        }
    }

    unsafe fn clear_color(color: &Color) {
        let &Color{ r, g, b, a } = color;
        gl::ClearColor(r, g, b, a);
    }

    pub fn run(&mut self) {
        self.sketch.setup();

        unsafe {
            self.gl_window.make_current().unwrap();
        }

        unsafe {
            gl::load_with(|symbol| self.gl_window.get_proc_address(symbol) as *const _);
            App::clear_color(&self.background);
        }

        let mut running = true;
        while running {
            let width = &mut self.width;
            let height = &mut self.height;
            let gl_window = &self.gl_window;
            self.events_loop.poll_events(|event| match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::Resized(w, h) => {
                        *width = w;
                        *height = h;
                        gl_window.resize(w, h)
                    },
                    _ => (),
                },
                _ => (),
            });

            unsafe {
                App::clear_color(&self.background);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                self.sketch.draw();
            }

            self.gl_window.swap_buffers().unwrap();
        }
    }
}
