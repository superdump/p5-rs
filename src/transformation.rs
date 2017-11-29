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

use std::fmt;
use std::sync::Mutex;

lazy_static! {
    static ref TRANSFORMATION_STACK: Mutex<Vec<Transformations>> = Mutex::new(vec![Transformations::new()]);
}

#[derive(Clone, Debug)]
pub struct Transformations {
    scale: Matrix,
    rotate: Matrix,
    translate: Matrix,
}

impl Transformations {
    pub fn new() -> Transformations {
        Transformations {
            scale: Matrix::identity(4, 4),
            rotate: Matrix::identity(4, 4),
            translate: Matrix::identity(4, 4),
        }
    }
    pub fn getMatrix(&self) -> Matrix {
        let scale = &self.scale;
        let mut rotate = self.rotate.clone();
        let mut translate = self.translate.clone();
        rotate.mul(scale);
        translate.mul(&rotate);
        translate
    }
    pub fn scale(&mut self, factors: Point) {
        self.scale.scale(factors);
    }
    pub fn shearX(&mut self, offset: f32) {
        self.scale.shearX(offset);
    }
    pub fn shearY(&mut self, offset: f32) {
        self.scale.shearY(offset);
    }
    pub fn rotate(&mut self, angle: f32) {
        self.rotate.rotateZ(angle);
    }
    pub fn rotateX(&mut self, angle: f32) {
        self.rotate.rotateX(angle);
    }
    pub fn rotateY(&mut self, angle: f32) {
        self.rotate.rotateY(angle);
    }
    pub fn rotateZ(&mut self, angle: f32) {
        self.rotate.rotateZ(angle);
    }
    pub fn translate(&mut self, p: Point) {
        self.translate.translate(p);
    }
}

impl fmt::Display for Transformations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "scale:\n{}", self.scale)?;
        write!(f, "rotate:\n{}", self.rotate)?;
        write!(f, "translate:\n{}", self.translate)
    }
}

pub fn reset() {
    let mut transformation_stack = TRANSFORMATION_STACK.lock().unwrap();
    transformation_stack.truncate(0);
    transformation_stack.push(Transformations::new());
}

pub fn getTransformations() -> Transformations {
    if let Some(transformations) = TRANSFORMATION_STACK.lock().unwrap().last() {
        return transformations.clone();
    }
    Transformations::new()
}

pub fn pushMatrix() {
    let mut transformation_stack = TRANSFORMATION_STACK.lock().unwrap();
    let mut clone = Transformations::new();
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
