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

use point::*;
use shape;
use shape::Shape;

pub fn triangle<P: Into<Point>>(left: P, right: P, top: P) {
    Triangle::new(left.into(), right.into(), top.into()).draw();
}

pub struct Triangle {
    points: Vec<Point>,
}

impl Triangle {
    pub fn new(left: Point, right: Point, top: Point) -> Triangle {
        Triangle {
            points: vec![left, right, top],
        }
    }
}

impl Shape for Triangle {
    fn points(&self) -> Vec<Point> {
        self.points.clone()
    }
    fn uvs(&self) -> Vec<f32> {
        // FIXME: naive left, right, top
        // Maybe find corners within a bounding rectangle?
        vec![0.0, 0.0, 1.0, 0.0, 0.5, 1.0]
    }
    fn indices(&self) -> Vec<u32> {
        vec![0, 1, 2]
    }
    fn vertex_shader(&self) -> String {
        String::from("")
    }
    fn fragment_shader(&self) -> String {
        String::from("")
    }
    fn draw(&self) {
        shape::draw(self);
    }
    fn is_stroke(&self) -> bool {
        false
    }
}
