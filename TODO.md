## TODO

* Optimizations
    * Tested hack that uploads geometry once and reuses it every frame
        * Enables 10x more triangles: 100k triangles at 10fps, 1M triangles at 1fps

* Infrastructure
    * `loop()`, `noLoop()`
* Imperative drawing
    * `beginShape()`, `vertex()`, `endShape()`
    * 2D: `quad()`, `arc()`
    * 3D: `box()`, `sphere()`, 2-sided 2D primitives
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
    * Check winding order of triangle vertices?
        * Not needed if we always draw both sides.
        * If a sketch had a 2D mode then we could only draw the face pointing in positive z.
* Optimizations
    * Avoid re-compiling and re-linking default shaders
    * Limit to VSync?
    * Channel incurs many context switches? Queue up closures and dispatch in one go.
    * Upload all geometry data for a single frame in one vertex and one index buffer
        * 10k triangles went from ~3fps to ~10fps
    * Draw using triangle strips instead of triangles. This reduces the number of calls to glDrawElements significantly for complex shapes such as ellipses.
        * 10k ellipses went from ~1.8fps to ~6fps
    * Draw ellipses using a rectangle and fragment shader
    * Cache shader programs
    * LRU cache for shapes
        * 10k triangles up to 15fps
        * 10k ellipses up to 15fps
* Infrastructure
    * `setup()`, `draw()`
* Global state
    * `size()`
    * `background()`
    * `fill()`, `noFill()`
    * `stroke()`, `noStroke()`, `strokeWeight()`
* Imperative drawing
    * 2D: `triangle()`, `rect()`, `ellipse()`, `point()`, `line()`