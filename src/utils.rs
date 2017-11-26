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

use point::Point;

use std::f32;

pub fn map_f32(iv: f32, il: f32, iu: f32, ol: f32, ou: f32) -> f32 {
    ol + (ou - ol) * (iv - il) / (iu - il)
}

pub fn map_f64(iv: f64, il: f64, iu: f64, ol: f64, ou: f64) -> f64 {
    ol + (ou - ol) * (iv - il) / (iu - il)
}

pub fn have_anticlockwise_winding(p1: Point, p2: Point, p3: Point) -> bool {
    let a = p1 - p2;
    let b = p3 - p2;
    // atan2 gives the anti-clockwise rotation about the z-axis from the x-axis
    let atana = a.y.atan2(a.x);
    let atanb = b.y.atan2(b.x);
    if atanb > 0.0 {
        return atana > atanb || atana + f32::consts::PI < atanb
    }
    atana > atanb && atana - f32::consts::PI < atanb
}

// returns left, top, right, bottom
pub fn bounding_box(points: &Vec<Point>) -> (f32, f32, f32, f32) {
    let mut left = f32::MAX;
    let mut bottom = f32::MAX;
    let mut right = f32::MIN;
    let mut top = f32::MIN;

    for point in points {
        if point.x < left {
            left = point.x;
        }
        if point.y < bottom {
            bottom = point.y;
        }
        if point.x > right {
            right = point.x;
        }
        if point.y > top {
            top = point.y;
        }
    }

    (left, top, right, bottom)
}