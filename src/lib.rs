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

#![feature(fnbox)]
#![feature(refcell_replace_swap)]

extern crate gl;
#[macro_use]
extern crate lazy_static;

mod channel;
mod color;
mod ellipse;
mod glapp;
mod point;
mod rectangle;
mod shader;
mod shape;
mod sketch;
mod triangle;
mod utils;

pub use color::*;
pub use ellipse::*;
pub use glapp::size;
pub use point::*;
pub use rectangle::*;
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
