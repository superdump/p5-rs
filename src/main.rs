extern crate p5;

use p5::*;

static mut t: f32 = 0.0;
static p1: Point = Point {
    x: -50.0,
    y: -50.0,
    z: 0.0,
};
static p2: Point = Point {
    x: 0.0,
    y: 43.0,
    z: 0.0,
};
static p3: Point = Point {
    x: 50.0,
    y: -50.0,
    z: 0.0,
};

fn setup() {
    size(400, 400);
    background(Color {
        r: 0.2,
        g: 0.2,
        b: 0.2,
        a: 1.0,
    });
}

fn point_on_circle(center: &Point, radius: f32, sin: f32, cos: f32) -> Point {
    Point {
        x: center.x + radius * sin,
        y: center.y + radius * cos,
        z: center.z,
    }
}

fn draw() {
    let origin = Point::new(0.0, 0.0, 0.0);
    let radius: f32 = 100.0;
    let sin: f32;
    let cos: f32;
    unsafe {
        sin = t.sin();
        cos = t.cos();
        t += 0.03;
    }
    triangle(
        point_on_circle(&origin, radius, sin, cos) + p1,
        point_on_circle(&origin, radius, sin, cos) + p2,
        point_on_circle(&origin, radius, sin, cos) + p3,
    );
}

fn main() {
    run_sketch(setup, draw);
}
