use point::*;
use shape;
use shape::Shape;

pub fn rect<P: Into<Point>>(top_left: P, bottom_right: P) {
    Rectangle::new(top_left.into(), bottom_right.into()).draw();
}

pub struct Rectangle {
    points: Vec<Point>,
}

impl Rectangle {
    pub fn new(top_left: Point, bottom_right: Point) -> Rectangle {
        let top_right = Point { x: bottom_right.x, y: top_left.y, z: top_left.z };
        let bottom_left = Point { x: top_left.x, y: bottom_right.y, z: bottom_right.z };
        Rectangle {
            points: vec![top_left, bottom_right, top_right, bottom_left],
        }
    }
}

impl Shape for Rectangle {
    fn points(&self) -> Vec<Point> {
        self.points.clone()
    }
    fn indices(&self) -> Vec<Vec<u32>> {
        vec!(vec!(0, 1, 2), vec!(0, 3, 1))
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
}
