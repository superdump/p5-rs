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

/*
 *  NOTE: These examples are entirely inspired by:
 * https://necessarydisorder.wordpress.com/2017/11/15/drawing-from-noise-and-then-making-animated-loopy-gifs-from-there/
 * All credit to that article and its author.
 */

extern crate noise;
extern crate p5;

use p5::*;

use std::f32;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

const N_POINTS: usize = 10_000;
static mut POINTS: Option<Vec<Point3<f32>>> = None;
static mut T: f32 = 0.0;

fn setup() {
    size(WIDTH, HEIGHT);
    background(0.2);

    let mut points: Vec<Point3<f32>> = vec![];
    for _ in 0..N_POINTS {
        let r = 0.5 * WIDTH as f32 * random(0.0, 1.0).sqrt();
        let angle = random(0.0, 2.0 * f32::consts::PI);
        points.push(Point3::new(r * angle.cos(), r * angle.sin(), 0.0));
    }
    unsafe {
        POINTS = Some(points);
    }
}

const RADIUS: f32 = 1.0;

fn draw() {
    stroke((1.0, 1.0, 1.0, 0.3));
    stroke_weight(1);

    // 1D - 0.01
    // stroke_weight(2);
    // const SCALE: f32 = 0.03;
    // for x in 0..WIDTH {
    //     let y = HEIGHT as f32 * noise(Point4::new(
    //         SCALE * x as f32,
    //         0.0,
    //         0.0,
    //         0.0,
    //     ));
    //     point(Point3::new(
    //         x as f32 - 0.5 * WIDTH as f32,
    //         y as f32 - 0.5 * HEIGHT as f32,
    //         0.0,
    //     ));
    // }

    // 2D
    // stroke_weight(1);
    // const SCALE: f32 = 0.01;
    // for y in 0..HEIGHT {
    //     for x in 0..WIDTH {
    //         let color = noise(Point4::new(
    //             x as f32 * SCALE,
    //             y as f32 * SCALE,
    //             0.0, 0.0,
    //         ));
    //         stroke(color);
    //         point(Point3::new(
    //             x as f32 - 0.5 * WIDTH as f32,
    //             y as f32 - 0.5 * HEIGHT as f32,
    //             0.0,
    //         ));
    //     }
    // }

    // 2D + time
    // let t;
    // unsafe {
    //     t = T;
    //     T += 1.0 / 75.0;
    // }
    // stroke_weight(1);
    // const SCALE: f32 = 0.01;
    // for y in 0..HEIGHT {
    //     for x in 0..WIDTH {
    //         let color = noise(Point4::new(
    //             x as f32 * SCALE,
    //             y as f32 * SCALE,
    //             t, 0.0,
    //         ));
    //         stroke(color);
    //         point(Point3::new(
    //             x as f32 - 0.5 * WIDTH as f32,
    //             y as f32 - 0.5 * HEIGHT as f32,
    //             0.0,
    //         ));
    //     }
    // }

    // 2D with perlin and thresholding
    // stroke_weight(4);
    // const SCALE: f32 = 0.05;
    // for y in 0..HEIGHT/4 {
    //     for x in 0..WIDTH/4 {
    //         let n = noise(Point4::new(
    //             x as f32 * SCALE,
    //             y as f32 * SCALE,
    //             0.0, 0.0,
    //         ));
    //         let mut c = 1.0;
    //         if n > 0.5 {
    //             c = 0.0;
    //         }
    //         stroke(c);
    //         point(Point3::new(
    //             4.0 * (x as f32 - 0.5 * (WIDTH / 4) as f32),
    //             4.0 * (y as f32 - 0.5 * (HEIGHT / 4) as f32),
    //             0.0,
    //         ));
    //     }
    // }

    // 2D + time with perlin and thresholding
    // let t;
    // unsafe {
    //     t = T;
    //     T += 1.0 / 300.0;
    // }
    // stroke_weight(4);
    // const SCALE: f32 = 0.04;
    // let angle = 2.0 * f32::consts::PI * t;
    // let rcos = RADIUS * angle.cos();
    // let rsin = RADIUS * angle.sin();
    // for y in 0..HEIGHT/4 {
    //     for x in 0..WIDTH/4 {
    //         let n = noise(Point4::new(
    //             x as f32 * SCALE,
    //             y as f32 * SCALE,
    //             rcos, rsin,
    //         ));
    //         let mut c = 1.0;
    //         if n > 0.5 {
    //             c = 0.0;
    //         }
    //         stroke(c);
    //         point(Point3::new(
    //             4.0 * (x as f32 - 0.5 * (WIDTH / 4) as f32),
    //             4.0 * (y as f32 - 0.5 * (HEIGHT / 4) as f32),
    //             0.0,
    //         ));
    //     }
    // }

    // 1D + time with perlin
    // let t;
    // unsafe {
    //     t = T;
    //     T += 1.0 / 300.0;
    // }
    // stroke_weight(1);
    // const SCALE: f32 = 0.01;
    // let angle = 2.0 * f32::consts::PI * t;
    // let rcos = RADIUS * angle.cos();
    // let rsin = RADIUS * angle.sin();
    // stroke_weight(4);
    // for x in 0..WIDTH {
    //     let n = 2.0 * (noise(Point4::new(
    //         x as f32 * SCALE,
    //         rcos, rsin, 0.0,
    //     )) - 0.5);
    //     let y = HEIGHT as f32 * n;
    //     point(Point3::new(
    //         x as f32 - 0.5 * WIDTH as f32,
    //         0.5 * y as f32,
    //         0.0,
    //     ));
    // }

    // 2D + time with perlin
    // let t;
    // unsafe {
    //     t = T;
    //     T += 1.0 / 300.0;
    // }
    // stroke_weight(4);
    // const SCALE: f32 = 0.05;
    // let angle = 2.0 * f32::consts::PI * t;
    // let rcos = RADIUS * angle.cos();
    // let rsin = RADIUS * angle.sin();
    // for y in 0..HEIGHT/4 {
    //     for x in 0..WIDTH/4 {
    //         let color = noise(Point4::new(
    //             x as f32 * SCALE,
    //             y as f32 * SCALE,
    //             rcos, rsin,
    //         ));
    //         stroke(color);
    //         point(Point3::new(
    //             4.0 * (x as f32 - 0.5 * (WIDTH / 4) as f32),
    //             4.0 * (y as f32 - 0.5 * (HEIGHT / 4) as f32),
    //             0.0,
    //         ));
    //     }
    // }

    let t;
    unsafe {
        t = T;
        T += 1.0 / 300.0;
    }
    stroke_weight(2);
    const SCALE: f32 = 0.005;
    let angle = 2.0 * f32::consts::PI * t;
    let rcos = RADIUS * angle.cos();
    let rsin = RADIUS * angle.sin();
    unsafe {
        if let Some(ref points) = POINTS {
            for p in points {
                let intensity = 250.0
                    * map_f32(
                        distance(p, &Point3::origin()),
                        0.0,
                        WIDTH as f32 * 0.5,
                        1.0,
                        0.0,
                    );
                point(Point3::new(
                    p.x + intensity * (noise(Point4::new(p.x * SCALE, p.y * SCALE, rcos, rsin)) - 0.5),
                    p.y
                        + intensity
                            * (noise(Point4::new(100.0 + p.x * SCALE, p.y * SCALE, rcos, rsin)) - 0.5),
                    0.0,
                ));
            }
        }
    }
}

fn main() {
    run_sketch(setup, draw, true);
}
