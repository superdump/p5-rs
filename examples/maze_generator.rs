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

extern crate p5;
extern crate rand;

use p5::*;
use rand::Rng;

#[repr(usize)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Cell {
    corners: Vec<Point3<f32>>,
    walls: Vec<bool>,
}

impl Cell {
    fn new(top_left: Point3<f32>, width: f32, height: f32) -> Cell {
        let right: Vector3<f32> = Vector3::new(width, 0.0, 0.0);
        let down: Vector3<f32> = Vector3::new(0.0, -height, 0.0);
        let top_right = top_left + right;
        let bottom_right = top_left + right + down;
        let bottom_left = top_left + down;
        Cell {
            corners: vec![top_left, top_right, bottom_right, bottom_left],
            walls: vec![true, true, true, true],
        }
    }
    fn break_wall(&mut self, wall: Direction) {
        self.walls[wall as usize] = false;
    }
    fn show(&self) {
        rect(self.corners[0], self.corners[2]);
        for i in 0..self.walls.len() {
            if self.walls[i] {
                line(self.corners[i], self.corners[(i + 1) % self.corners.len()]);
            }
        }
    }
}

fn out_of_bounds(i: i32, j: i32, w: i32, h: i32) -> bool {
    i < 0 || i >= w || j < 0 || j >= h
}

#[derive(Clone, Copy, Debug)]
struct Index {
    i: i32,
    j: i32,
}

impl Index {
    fn new(i: i32, j: i32) -> Index {
        Index { i, j }
    }
    fn index(&self) -> usize {
        index(self.i, self.j)
    }
    fn up(&self) -> Index {
        Index { i: self.i, j: self.j - 1 }
    }
    fn right(&self) -> Index {
        Index { i: self.i + 1, j: self.j }
    }
    fn down(&self) -> Index {
        Index { i: self.i, j: self.j + 1 }
    }
    fn left(&self) -> Index {
        Index { i: self.i - 1, j: self.j }
    }
}

#[derive(Debug)]
struct Grid {
    cols: i32,
    rows: i32,
    grid: Vec<Cell>,
    visited: Vec<bool>,
    stack: Vec<Index>,
    current: Index,
    generating: bool,
}

impl Grid {
    fn new(cols: i32, rows: i32, cw: i32, ch: i32) -> Grid {
        let mut grid = Vec::with_capacity((cols * rows) as usize);
        for j in 0..rows {
            for i in 0..cols {
                grid.push(Cell::new(cell_to_point(i, j), cw as f32, ch as f32));
            }
        }
        Grid {
            cols,
            rows,
            grid,
            visited: Vec::new(),
            stack: Vec::new(),
            current: Index::new(-1, -1),
            generating: false,
        }
    }
    fn show(&self) {
        let mut index = 0;
        for j in 0..self.rows {
            for i in 0..self.cols {
                if self.current.i == i && self.current.j == j {
                    fill((0.0, 1.0, 0.0, 0.7));
                } else if !self.visited.is_empty() && self.visited[index] {
                    fill((0.0, 0.0, 1.0, 0.7));
                } else {
                    noFill();
                }
                self.grid[index].show();
                index += 1;
            }
        }
    }
    fn maze_gen_start(&mut self, i: i32, j: i32) {
        if out_of_bounds(i, j, self.cols, self.rows) {
            println!("Cannot start maze generation at ({},{})", i, j);
            return
        }
        self.visited = vec![false; (self.cols * self.rows) as usize];
        self.stack = Vec::new();
        self.current = Index::new(i, j);
        self.visited[self.current.index()] = true;
        self.generating = true;
    }
    fn maze_gen_step(&mut self) {
        if !self.generating {
            return
        }
        let neighbors = self.get_neighbors(self.current);
        if neighbors.is_empty() {
            if self.stack.is_empty() {
                self.visited = vec![false; (self.cols * self.rows) as usize];
                self.current = Index::new(-1, -1);
                println!("Maze generated");
                self.generating = false;
                return
            }
            if let Some(current) = self.stack.pop() {
                self.current = current;
                return
            }
        }
        if let Some(next) = rand::thread_rng().choose(&neighbors) {
            let current = self.current;
            self.stack.push(current);
            self.visited[next.index()] = true;
            self.break_walls(current, *next);
            self.current = *next;
        }
    }
    fn check_neighbor(&self, cell: Index) -> Option<Index> {
        if !out_of_bounds(cell.i, cell.j, self.cols, self.rows) && !self.visited[cell.index()] {
            return Some(cell);
        }
        None
    }
    fn get_neighbors(&self, cell: Index) -> Vec<Index> {
        let mut neighbors: Vec<Index> = Vec::new();
        if let Some(up) = self.check_neighbor(cell.up()) {
            neighbors.push(up);
        }
        if let Some(right) = self.check_neighbor(cell.right()) {
            neighbors.push(right);
        }
        if let Some(down) = self.check_neighbor(cell.down()) {
            neighbors.push(down);
        }
        if let Some(left) = self.check_neighbor(cell.left()) {
            neighbors.push(left);
        }
        neighbors
    }
    fn break_walls(&mut self, prev: Index, curr: Index) {
        if out_of_bounds(prev.i, prev.j, self.cols, self.rows) {
            return
        }
        if prev.j == curr.j {
            if prev.i < curr.i {
                self.grid[prev.index()].break_wall(Direction::Right);
                self.grid[curr.index()].break_wall(Direction::Left);
            } else {
                self.grid[prev.index()].break_wall(Direction::Left);
                self.grid[curr.index()].break_wall(Direction::Right);
            }
        } else {
            if prev.j < curr.j {
                self.grid[prev.index()].break_wall(Direction::Down);
                self.grid[curr.index()].break_wall(Direction::Up);
            } else {
                self.grid[prev.index()].break_wall(Direction::Up);
                self.grid[curr.index()].break_wall(Direction::Down);
            }
        }

    }
}

const WIDTH: i32 = 800;
const HEIGHT: i32 = WIDTH;
const CELL_WIDTH: i32 = WIDTH/40;
const CELL_HEIGHT: i32 = CELL_WIDTH;
const COLS: i32 = WIDTH / CELL_WIDTH;
const ROWS: i32 = COLS;
const N_CELLS: usize = COLS as usize * ROWS as usize;

static mut GRID: Option<Grid> = None;

fn index(i: i32, j: i32) -> usize {
    (j * COLS + i) as usize
}

fn cell_to_point(i: i32, j: i32) -> Point3<f32> {
    Point3::new(
        (-WIDTH/2 + i * CELL_WIDTH) as f32,
        (HEIGHT/2 - j * CELL_HEIGHT) as f32,
        0.0,
    )
}

fn setup() {
    size(800, 800);
    background(0.2);
    unsafe {
        GRID = Some(Grid::new(COLS, ROWS, CELL_WIDTH, CELL_HEIGHT));
        if let Some(ref mut grid) = GRID {
            grid.maze_gen_start(0, 0);
        }
    }
}

fn draw() {
    strokeWeight(1);
    stroke(1.0);
    unsafe {
        if let Some(ref mut grid) = GRID {
            grid.maze_gen_step();
            grid.show();
        }
    }
}

fn main() {
    run_sketch(setup, draw, true);
}
