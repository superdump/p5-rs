## TODO

* Optimizations
    * Channel incurs many context switches? Queue up closures and dispatch in one go.
* Infrastructure
    * `loop()`, `noLoop()`
* Global state
    * `stroke()`, `noStroke()`, `strokeWeight()`
        * Cannot just use a fragment shader?
    * `fill()`, `noFill()`
* Imperative drawing
    * `beginShape()`, `vertex()`, `endShape()`
    * 2D: `ellipse()`, `line()`, `point()`, `quad()`, `arc()`
    * 3D: `box()`, `sphere()`
    * transforms: `popMatrix()`, `pushMatrix()`, `rotate()`, `rotateX()`, `rotateY()`, `rotateZ()`, `translate()`
* Color
    * HSB, HSL, RGB, alpha - `colorMode()`
* Utility
    * `dist()`, `lerp()`, `mag()`, `map()`, `norm()`
    * `degrees()`, `radians()`
    * `noise()`, `random()`

## DONE

* Bugs
    * Closing the window and pressing escape should gracefully quit the application
* Optimizations
    * Avoid re-compiling and re-linking default shaders
    * Limit to VSync?
* Infrastructure
    * `setup()`, `draw()`
* Global state
    * `size()`
    * `background()`
* Imperative drawing
    * 2D: `triangle()`, `rect()`