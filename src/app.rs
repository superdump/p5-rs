extern crate gl;
extern crate glutin;
extern crate libc;

use self::glutin::GlContext;

use color::*;
use sketch::*;

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
            background: Color{ r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
            events_loop: events_loop,
            gl_window: gl_window
        }
    }

    pub fn background(color: &Color) {
        let &Color{ r, g, b, a } = color;
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn run(&mut self) {
        unsafe {
            self.gl_window.make_current().unwrap();
        }

        gl::load_with(|symbol| self.gl_window.get_proc_address(symbol) as *const _);
        App::background(&self.background);

        self.sketch.setup();

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

            self.sketch.draw();

            self.gl_window.swap_buffers().unwrap();
        }
    }
}
