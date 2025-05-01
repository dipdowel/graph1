# The Roadmap

**NB:** The roadmap is still a work in progress. The notes below might be outdated and/or incomplete.
- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 

## TODO (partially outdated?)
- [ ] Get rid of &u32 and other primitive numeric types passed by reference. That's inefficient.
-[ ] Create a package with formulas, like formulas of circle, ellipse, etc.
 ================================================================================================================
-[ ] Consider adding `#[derive(Default)]` in places where it makes sense!
-[ ] Explore `Default`: .../stdlib-local-copy/bundled-1.82.0/library/core/src/default.rs


## Luminance and Intensity

## Intensity

## Documentation
Update the documentation with the multithreaded support in the operations.

## multithreaded support in operations
- Make a multithreaded function that copies one buffer to another
- Drawing a rectangle can be multithreaded, try it!
- Make multithreaded color adapters
- The scanline effect can potentially be multithreaded
- Check if flood fill can be multithreaded