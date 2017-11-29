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

extern crate game_time;
extern crate gl;
#[macro_use]
extern crate lazy_static;
extern crate ordered_float;

mod channel;
mod color;
mod ellipse;
mod glapp;
mod line;
mod matrix;
mod point;
mod rectangle;
mod shader;
mod shape;
mod sketch;
mod transformation;
mod triangle;
mod utils;

pub use color::*;
pub use ellipse::*;
pub use glapp::size;
pub use line::*;
pub use matrix::*;
pub use point::*;
pub use rectangle::*;
pub use shader::*;
pub use shape::*;
pub use sketch::*;
pub use transformation::*;
pub use triangle::*;
pub use utils::*;

use game_time::*;

use std::thread;

pub fn run_sketch(setup: fn(), draw: fn(), log: bool) {
    let rx = channel::make_channel();

    glapp::setup();
    thread::spawn(move || {
        setup();

        let mut clock = GameClock::new();
        let mut counter = FrameCounter::new(
            60.0,
            framerate::sample::LinearAverageSampler::with_max_samples(60),
        );
        let mut time = clock.last_frame_time().clone();

        loop {
            // FIXME: need a backchannel for events from the event loop polling
            glapp::poll_events();

            if log {
                time = clock.tick(&step::FixedStep::new(&counter));
                counter.tick(&time);
            }
            transformation::reset();
            color::draw_background();
            draw();

            if log && time.frame_number() % 60 == 1 {
                println!(
                    "Frame #{} at sketch_time={:.3}s wall_time={:.3}s frame_time={:.3}ms fps(mean)={:.3} fps={:.3}",
                    time.frame_number(),
                    time.total_game_time().as_seconds(),
                    clock.total_wall_time().as_seconds(),
                    time.elapsed_time_since_frame_start().as_milliseconds(),
                    counter.average_frame_rate(),
                    time.instantaneous_frame_rate(),
                );
            }

            glapp::swap_buffers();
            channel::send();
        }
    });
    glapp::listen(rx);
}
