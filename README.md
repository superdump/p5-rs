# p5-rs - Processing / p5 for Rust

This is an attempt by an OpenGL / Rust newbie ([Robert Swain](https://github.com/superdump) 2017-11) to develop a library heading somewhat toward the same kind of functionality and API as Processing / p5 but for Rust. I'm doing this mostly for my education.

## Usage

Make a `main.rs` containing the following:

```
extern crate p5;

use p5::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn setup() {
    size(WIDTH, HEIGHT);
}

fn draw() {
}

fn main() {
    run_sketch(setup, draw, true);
}
```

As usual with Processing / p5, write code in and around the `setup()` and `draw()` functions to draw fun things.

## Design

The Processing and p5 APIs lead to a design with implicit global state that one modifies through API calls. For example, setting the fill color will result in all fillable entities being filled with that color until it is changed.

There is high-level infrastructure for handling initialization (the `setup()` callback function) and the render loop (the `draw()` callback function). These are to be implemented by the user and form the main entry points of a so-called 'sketch' into which the Processing/p5 engine calls. A sketch is the name for a creation made using the framework.

Most of the interesting part of a sketch is initiated through the `draw()` callback which is called once per frame to be rendered to the display. It is through code paths from this function that one draws everything to the display, such as rectangles, spheres, lines, points and so on.

In order to make more interesting sketches, it is common to use a measurement of time that is incremented once per frame and then draw entities based on that time value. This approach can be used to step through animations or algorithms to progress the visualisation.

## License

MIT
