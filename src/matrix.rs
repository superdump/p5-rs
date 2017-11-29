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
use utils::index;

use std::fmt;

#[derive(Clone, Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    matrix: Vec<f32>, // NOTE: stored in column-major order
}

impl Matrix {
    pub fn new(w: usize, h: usize) -> Matrix {
        Matrix {
            rows: h,
            cols: w,
            matrix: vec![0.0; w * h],
        }
    }
    pub fn as_vec(&self) -> Vec<f32> {
        self.matrix.clone()
    }
    pub fn out_of_bounds(&self, row: usize, col: usize) -> bool {
        row >= self.rows || col >= self.cols
    }
    pub fn identity(w: usize, h: usize) -> Matrix {
        let mut matrix = Matrix::new(w, h);
        let mut ij = 0;
        while !matrix.out_of_bounds(ij, ij) {
            matrix.matrix[index(ij, ij, matrix.rows)] = 1.0;
            ij += 1;
        }
        matrix
    }
    pub fn mul(&mut self, m: &Matrix) {
        if self.cols != m.rows {
            return;
        }
        let mut result = Vec::new();
        for col in 0..m.cols {
            for row in 0..self.rows {
                let mut sum = 0.0;
                for i in 0..self.cols {
                    sum += self.matrix[index(i, row, 4)] * m.matrix[index(col, i, 4)];
                }
                result.push(sum);
            }
        }
        self.cols = m.cols;
        self.matrix = result;
    }
    pub fn pre_mul(&mut self, m: &Matrix) {
        if m.cols != self.rows {
            return;
        }
        let mut result = Vec::new();
        for col in 0..self.cols {
            for row in 0..m.rows {
                let mut sum = 0.0;
                for i in 0..m.cols {
                    sum += m.matrix[index(i, row, 4)] * self.matrix[index(col, i, 4)];
                }
                result.push(sum);
            }
        }
        self.cols = m.cols;
        self.matrix = result;
    }
    pub fn translate(&mut self, p: Point) {
        let mut transform = Matrix::identity(4, 4);
        let index = index(3, 0, 4);
        transform.matrix[index + 0] += p.x;
        transform.matrix[index + 1] += p.y;
        transform.matrix[index + 2] += p.z;
        self.pre_mul(&transform);
    }
    pub fn rotateX(&mut self, angle: f32) {
        let mut transform = Matrix::identity(4, 4);
        let cos = angle.cos();
        let sin = angle.sin();
        transform.matrix[index(1, 1, 4)] = cos;
        transform.matrix[index(2, 1, 4)] = -sin;
        transform.matrix[index(1, 2, 4)] = sin;
        transform.matrix[index(2, 2, 4)] = cos;
        self.pre_mul(&transform);
    }
    pub fn rotateY(&mut self, angle: f32) {
        let mut transform = Matrix::identity(4, 4);
        let cos = angle.cos();
        let sin = angle.sin();
        transform.matrix[index(0, 0, 4)] = cos;
        transform.matrix[index(2, 0, 4)] = sin;
        transform.matrix[index(0, 2, 4)] = -sin;
        transform.matrix[index(2, 2, 4)] = cos;
        self.pre_mul(&transform);
    }
    pub fn rotateZ(&mut self, angle: f32) {
        let mut transform = Matrix::identity(4, 4);
        let cos = angle.cos();
        let sin = angle.sin();
        transform.matrix[index(0, 0, 4)] = cos;
        transform.matrix[index(1, 0, 4)] = -sin;
        transform.matrix[index(0, 1, 4)] = sin;
        transform.matrix[index(1, 1, 4)] = cos;
        self.pre_mul(&transform);
    }
    pub fn scale(&mut self, factors: Point) {
        let mut transform = Matrix::identity(4, 4);
        transform.matrix[index(0, 0, 4)] *= factors.x;
        transform.matrix[index(1, 1, 4)] *= factors.y;
        transform.matrix[index(2, 2, 4)] *= factors.z;
        self.pre_mul(&transform);
    }
    pub fn shearX(&mut self, offset: f32) {
        let mut transform = Matrix::identity(4, 4);
        transform.matrix[index(1, 0, 4)] = offset;
        self.pre_mul(&transform);
    }
    pub fn shearY(&mut self, offset: f32) {
        let mut transform = Matrix::identity(4, 4);
        transform.matrix[index(0, 1, 4)] = offset;
        self.pre_mul(&transform);
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.rows {
            write!(f, "[ ")?;
            for j in 0..self.cols {
                if j != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:8.3}", self.matrix[index(j, i, self.rows)])?;
            }
            writeln!(f, " ]")?;
        }
        Ok(())
    }
}