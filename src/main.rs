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
        App::background(&Color{r: 0.2, g: 0.2, b: 0.2, a: 1.0});
    }

    fn draw(&mut self) {
        let v = 0.2;
        App::background(&Color{r: v, g: v, b: v, a: 1.0});
    }
}

fn main() {
    let mut sketch = MySketch::new();
    let mut app = App::new(&mut sketch);
    app.run();
}