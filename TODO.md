-[ ] Get rid of &u32 and other primitive numeric types passed by reference. That's inefficient.
-[ ] Get rid of f64 in favor of f32 (cache usage) https://hugotuniusenu.se/2017/12/04/rust-f64-vs-f32.html
-[ ] Create a package with formulas, like formulas of circle, ellipse, etc.
 ================================================================================================================
-[ ] Consider adding `#[derive(Default)]` in places where it makes sense!
-[ ] Explode `Default`: .../stdlib-local-copy/bundled-1.82.0/library/core/src/default.rs


## Alpha and blending:
- [ ] consider using `AlphaMethod::None` instead of `AlphaContext::enabled`
  - AlphaMethod::None
  - AlphaMethod::Int
  - AlphaMethod::Float
- **NB:** This might not be the best idea, since it removes a simple way to toggle alpha blending on and off.  
  

- [ ] consider renaming `AlphaMethod` to `BlendingMethod`, so that: 
  - BlendingMethod::None
  - BlendingMethod::Alpha_Int
  - BlendingMethod::Alpha_Float
  - BlendingMethod::Color
  - Etc.

## Luminance and Intensity
- [ ] Make both of them return `u32` and operate on `u32` buffers
- [ ] Make a channel extractor for `u32` buffers, which would convert a `u32` buffer to a `u8` of the same length

## Intensity
- [ ] Implement `square` parameter for all the functions, adjust the RUstDoc accordingly!


## Documentation
Update the documentation with the multithreaded support in the operations. 


## multithreaded support in operations
- Make a multithreaded function that copies one buffer to another 
- Add multithreaded support to buffer fill
- Drawing a rectangle can be multithreaded, try it!
- Make multithreaded color adapters
- The scanline effect can potentially be multithreaded
- Check if flood fill can be multithreaded