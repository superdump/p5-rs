extern crate glutin;
extern crate libc;

use std::cell::RefCell;

use self::glutin::GlContext;
use gl;

use color::*;

thread_local! {
    pub static RUST_THREAD_LOCAL: RefCell<App> = RefCell::new(App::new(None, None));
}

pub fn run_app(setup: fn(), draw: fn()) {
    RUST_THREAD_LOCAL.with(|handle| {
        let mut app = handle.borrow_mut();
        app.setup = Some(setup);
        app.draw = Some(draw);
        app.run();
    });
}

pub struct App {
    setup: Option<fn()>,
    draw: Option<fn()>,
    width: u32,
    height: u32,
    background: Color,
    events_loop: glutin::EventsLoop,
    gl_window: glutin::GlWindow
}

impl App {
    pub fn new(setup: Option<fn()>, draw: Option<fn()>) -> App {
        let w = 640;
        let h = 360;

        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title("p5-rs sketch")
            .with_dimensions(w, h);
        let context = glutin::ContextBuilder::new().with_vsync(true);
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        App {
            setup: setup,
            draw: draw,
            width: w,
            height: h,
            background: Color{ r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
            events_loop: events_loop,
            gl_window: gl_window
        }
    }

    pub fn run(&mut self) {
        unsafe {
            self.gl_window.make_current().unwrap();
        }

        gl::load_with(|symbol| self.gl_window.get_proc_address(symbol) as *const _);
        background(&self.background);

        if let Some(setup) = self.setup {
            setup();
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

            if let Some(draw) = self.draw {
                draw();
            }

            self.gl_window.swap_buffers().unwrap();
        }
    }
}

pub fn background(color: &Color) {
    let &Color{ r, g, b, a } = color;
    unsafe {
        gl::ClearColor(r, g, b, a);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}
