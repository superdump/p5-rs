use point::*;
use shader;
use shape;
use shape::Shape;

pub fn triangle<P: Into<Point>>(p1: P, p2: P, p3: P) {
    Triangle::new(p1.into(), p2.into(), p3.into()).draw();
}

pub struct Triangle {
    points: Vec<Point>,
    vertex_shader: String,
    fragment_shader: String
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Triangle {
        Triangle {
            points: vec![p1, p2, p3],
            vertex_shader: String::from(shader::DEFAULT_VERTEX_SHADER),
            fragment_shader: String::from(shader::DEFAULT_FRAGMENT_SHADER)
        }
    }
}

impl Shape for Triangle {
    fn points(&self) -> Vec<Point> {
        self.points.clone()
    }
    fn vertex_shader(&self) -> String {
        self.vertex_shader.clone()
    }
    fn fragment_shader(&self) -> String {
        self.fragment_shader.clone()
    }
    fn draw(&self) {
        shape::draw(self);
    }
}
