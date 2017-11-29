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

use matrix::*;
use point::Point;

use std::sync::Mutex;

lazy_static! {
    static ref TRANSFORMATION_STACK: Mutex<Vec<Matrix>> = Mutex::new(vec![Matrix::identity(4, 4)]);
}

pub fn reset() {
    let mut transformation_stack = TRANSFORMATION_STACK.lock().unwrap();
    transformation_stack.truncate(0);
    transformation_stack.push(Matrix::identity(4, 4));
}

pub fn getTransformations() -> Matrix {
    if let Some(transformations) = TRANSFORMATION_STACK.lock().unwrap().last() {
        return transformations.clone();
    }
    Matrix::identity(4, 4)
}

pub fn pushMatrix() {
    let mut transformation_stack = TRANSFORMATION_STACK.lock().unwrap();
    let mut clone = Matrix::identity(4, 4);
    if let Some(top) = transformation_stack.last() {
        clone = top.clone();
    }
    transformation_stack.push(clone);
}

pub fn popMatrix() {
    TRANSFORMATION_STACK.lock().unwrap().pop();
}

pub fn translate<P: Into<Point>>(translation: P) {
    if let Some(ref mut transformation) = TRANSFORMATION_STACK.lock().unwrap().last_mut() {
        transformation.translate(translation.into());
    }
}

pub fn rotate(angle: f32) {
    if let Some(ref mut transformation) = TRANSFORMATION_STACK.lock().unwrap().last_mut() {
        transformation.rotateZ(angle);
    }
}

pub fn rotateX(angle: f32) {
    if let Some(ref mut transformation) = TRANSFORMATION_STACK.lock().unwrap().last_mut() {
        transformation.rotateX(angle);
    }
}

pub fn rotateY(angle: f32) {
    if let Some(ref mut transformation) = TRANSFORMATION_STACK.lock().unwrap().last_mut() {
        transformation.rotateY(angle);
    }
}

pub fn rotateZ(angle: f32) {
    if let Some(ref mut transformation) = TRANSFORMATION_STACK.lock().unwrap().last_mut() {
        transformation.rotateZ(angle);
    }
}

pub fn scale<P: Into<Point>>(factor: P) {
    if let Some(ref mut transformation) = TRANSFORMATION_STACK.lock().unwrap().last_mut() {
        transformation.scale(factor.into());
    }
}
