extern crate p5;

struct MySketch {}

impl MySketch {
    fn new() -> MySketch {
        MySketch {}
    }
}

impl p5::sketch::Sketch for MySketch {
    fn setup(&mut self) {
    }

    fn draw(&mut self) {
    }
}

fn main() {
    let mut sketch = MySketch::new();
    let mut app = p5::App::new(&mut sketch);
    app.run();
}