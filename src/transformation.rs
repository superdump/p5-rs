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

use na::{Matrix4, Rotation3, Transform3, Translation, Vector3};

use std::sync::{Mutex, MutexGuard};

lazy_static! {
    static ref TRANSFORMATION_STACK: Mutex<Vec<Transform3<f32>>> = Mutex::new(vec![Transform3::identity()]);
}

pub fn reset() {
    let mut transformation_stack = TRANSFORMATION_STACK.lock().unwrap();
    transformation_stack.truncate(0);
    transformation_stack.push(Transform3::identity());
}

pub fn getTransformations<'a>() -> MutexGuard<'a, Vec<Transform3<f32>>> {
    TRANSFORMATION_STACK.lock().unwrap()
}

pub fn pushMatrix() {
    let mut transformation_stack = TRANSFORMATION_STACK.lock().unwrap();
    let clone;
    if let Some(top) = transformation_stack.last() {
        clone = top.clone();
    } else {
        clone = Transform3::identity();
    }
    transformation_stack.push(clone);
}

pub fn popMatrix() {
    TRANSFORMATION_STACK.lock().unwrap().pop();
}

pub fn translate(translation: &Vector3<f32>) {
    if let Some(transformation) = TRANSFORMATION_STACK.lock().unwrap().last_mut() {
        *transformation = Translation::from_vector(*translation) * *transformation;
    }
}

pub fn rotate(angle: f32) {
    rotateZ(angle);
}

pub fn rotateX(angle: f32) {
    if let Some(transformation) = TRANSFORMATION_STACK.lock().unwrap().last_mut() {
        *transformation = Rotation3::from_scaled_axis(&Vector3::x() * angle) * *transformation;
    }
}

pub fn rotateY(angle: f32) {
    if let Some(transformation) = TRANSFORMATION_STACK.lock().unwrap().last_mut() {
        *transformation = Rotation3::from_scaled_axis(&Vector3::y() * angle) * *transformation;
    }
}

pub fn rotateZ(angle: f32) {
    if let Some(transformation) = TRANSFORMATION_STACK.lock().unwrap().last_mut() {
        *transformation = Rotation3::from_scaled_axis(&Vector3::z() * angle) * *transformation;
    }
}

pub fn scale(factors: &Vector3<f32>) {
    if let Some(transformation) = TRANSFORMATION_STACK.lock().unwrap().last_mut() {
        *transformation = Transform3::from_matrix_unchecked(
            Matrix4::new_nonuniform_scaling(factors)
        ) * *transformation;
    }
}
