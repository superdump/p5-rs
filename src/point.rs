#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Point {
        Point {
            x: x,
            y: y,
            z: z
        }
    }
}