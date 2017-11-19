extern crate p5;

use p5::*;

fn setup() {
    size(400, 400);
    background(Color {
        r: 0.2,
        g: 0.2,
        b: 0.2,
        a: 1.0,
    });
}

fn draw() {
    triangle(
        Point::new(-50, 0, 0),
        Point::new(0, 87, 0),
        Point::new(50, 0, 0),
    );
}

fn main() {
    run_sketch(setup, draw);
}
