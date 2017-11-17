extern crate p5;

use p5::*;

struct MySketch {}

impl MySketch {
    fn new() -> MySketch {
        MySketch {}
    }
}

impl sketch::Sketch for MySketch {
    fn setup(&mut self) {
    }

    fn draw(&mut self) {
    }
}

fn main() {
    let mut sketch = MySketch::new();
    let mut app = App::new(&mut sketch);
    app.run();
}