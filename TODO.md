## TODO

* Infrastructure
    * `loop()`, `noLoop()`
* Imperative drawing
    * `beginShape()`, `vertex()`, `endShape()`
    * 2D: `quad()`, `arc()`
    * 3D: `box()`, `sphere()`, 2-sided 2D primitives
* Color
    * HSB, HSL, RGB, alpha - `colorMode()`
* Utility
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
    * Tested hack that uploads geometry once and reuses it every frame
        * Enables 10x more triangles: 100k triangles at 10fps, 1M triangles at 1fps
    * Draw using triangle strips instead of triangles. This reduces the number of calls to glDrawElements significantly for complex shapes such as ellipses.
        * 10k ellipses went from ~1.8fps to ~6fps
    * Draw ellipses using a rectangle and fragment shader
    * Cache shader programs
    * LRU cache for shapes
        * 10k triangles up to 15fps
        * 10k ellipses up to 15fps
        * Maze generator up from 10fps at the beginning and 19fps at the end to 20fps at the beginning and 30fps at the end. The speedup is due to walls being destroyed reducing the numnber of things to draw per frame.
    * Discussed performance issues with Jakub Valtar who works on Processing
        * Have to minimize draw calls
            * Now one for all geometry as everything is a triangle strip. Duplicated indices produce degenerate triangles allowing disjoint shapes.
        * Use few large vertex buffers
            * One large array buffer for vertex data, one element array buffer for indices
        * Use batch drawing (increase count of drawn shapes in glDrawElements)
            * One glDrawElements call
        * Avoid switching shader programs often as it stalls the GL rendering pipeline
            * Probably don't want to use a fragment shader for ellipses, or will have to render ellipses separately. Draw ellipses using triangle strip.
            * Currently only using one default shader. If more shaders are needed, those geometries will have to be rendered using separate draw calls in order to switch programs.
        * Generate buffers once and re-upload data by binding the existing buffer and using glBufferData
            * Now doing this.
        * RESULTS:
            * 10k triangles 60fps
            * 10k points (now 160k vertices) 60fps
            * Maze generator solid 60fps
    * ystreet0 (Matthew Waters) suggested to upload data in one frame and render it in the next - essentially double-buffering
        * Implemented but doesn't seem to help.
* Infrastructure
    * `setup()`, `draw()`
* Global state
    * `size()`
    * `background()`
    * `fill()`, `noFill()`
    * `stroke()`, `noStroke()`, `strokeWeight()`
* Imperative drawing
    * 2D: `triangle()`, `rect()`, `ellipse()`, `point()`, `line()`
    * transforms: `popMatrix()`, `pushMatrix()`, `rotate()`, `rotateX()`, `rotateY()`, `rotateZ()`, `translate()`, `scale()`
* Utility
    * (use nalgebra) `dist()`, `lerp()`, `mag()`, `map()`, `norm()`
