extern crate p5;

use p5::*;

fn setup() {
    background(Color {
        r: 0.2,
        g: 0.2,
        b: 0.2,
        a: 1.0,
    });
}

fn draw() {
    // let tri = Triangle::new(
    //     Point::new(-50, 0, 0),
    //     Point::new(0, 87, 0),
    //     Point::new(50, 0, 0),
    // );
    // tri.draw();
}

fn main() {
    run_sketch(setup, draw);
}
