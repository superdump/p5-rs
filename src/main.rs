extern crate p5;

use p5::*;

struct MySketch {
    tri: Triangle
}

impl MySketch {
    fn new() -> MySketch {
        MySketch {
            tri: Triangle::new(
                Point::new(-50, 0, 0),
                Point::new(0, 87, 0),
                Point::new(50, 0, 0)
            )
        }
    }
}

impl Sketch for MySketch {
    fn setup(&mut self) {
        background(&Color{r: 0.2, g: 0.2, b: 0.2, a: 1.0});
    }

    fn draw(&mut self) {
        let v = 0.2;
        background(&Color{r: v, g: v, b: v, a: 1.0});
        self.tri.draw();
    }
}

fn main() {
    let mut sketch = MySketch::new();
    let mut app = App::new(&mut sketch);
    app.run();
}