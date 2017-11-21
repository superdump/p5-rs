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

pub fn rect<P: Into<Point>>(top_left: P, bottom_right: P) {
    Rectangle::new(top_left.into(), bottom_right.into(), false).draw();
}

pub struct Rectangle {
    points: Vec<Point>,
    is_stroke: bool,
}

impl Rectangle {
    pub fn new(top_left: Point, bottom_right: Point, is_stroke: bool) -> Rectangle {
        Rectangle {
            points: vec![top_left, bottom_right, top_right, bottom_left],
            is_stroke,
        }
    }
}

impl Shape for Rectangle {
    fn points(&self) -> Vec<Point> {
        self.points.clone()
    }
    fn indices(&self) -> Vec<Vec<u32>> {
        vec![vec![0, 1, 2], vec![0, 3, 1]]
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
        self.is_stroke
    }
}
