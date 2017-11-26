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

use ellipse::*;
use shape::Shape;
use sketch::get_stroke_weight;

use std::ops::Add;

pub fn point<P: Into<Point>>(point: P) {
    let diameter = get_stroke_weight();
    Ellipse::new(point.into(), diameter as f32, diameter as f32, 16, true).draw();
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x: x, y: y, z: z }
    }
    pub fn mag(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn setMag(&mut self, new_mag: f32) {
        let scale = new_mag / self.mag();
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
    }
    pub fn normalize(&mut self) {
        self.setMag(1.0);
    }
}

impl From<(f32, f32)> for Point {
    fn from(p: (f32, f32)) -> Self {
        Point {
            x: p.0,
            y: p.1,
            z: 0.0,
        }
    }
}

impl From<(f32, f32, f32)> for Point {
    fn from(p: (f32, f32, f32)) -> Self {
        Point {
            x: p.0,
            y: p.1,
            z: p.2,
        }
    }
}

impl<'a, 'b> Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a> Add<&'a Point> for Point {
    type Output = Point;

    fn add(self, other: &'a Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}


impl<'a> Add<Point> for &'a Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
