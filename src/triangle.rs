use point::*;
use shape;
use shape::Shape;

pub fn triangle<P: Into<Point>>(p1: P, p2: P, p3: P) {
    Triangle::new(p1.into(), p2.into(), p3.into()).draw();
}

pub struct Triangle {
    points: Vec<Point>,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Triangle {
        Triangle {
            points: vec![p1, p2, p3],
        }
    }
}

impl Shape for Triangle {
    fn points(&self) -> Vec<Point> {
        self.points.clone()
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
