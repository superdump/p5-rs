#![feature(fnbox)]
#![feature(refcell_replace_swap)]

extern crate gl;
#[macro_use]
extern crate lazy_static;

mod channel;
mod color;
mod glapp;
mod point;
mod shader;
mod shape;
mod sketch;
mod triangle;
mod utils;

pub use color::*;
pub use point::*;
pub use shader::*;
pub use shape::*;
pub use sketch::*;
pub use triangle::*;
pub use utils::*;

use std::thread;

pub fn run_sketch(setup: fn(), draw: fn()) {
    let rx = channel::make_channel();

    glapp::setup();
    thread::spawn(move || {
        setup();

        let mut running = true;
        while running {
            // FIXME: need a backchannel for events from the event loop polling
            glapp::poll_events();
            draw_background();
            draw();

            glapp::swap_buffers();
        }
    });
    glapp::listen(rx);
}
