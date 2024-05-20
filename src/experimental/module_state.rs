
use std::cell::RefCell;
//
// enum ColorFillMode {
//     Color,
//     Transformer
// }
//
// struct PrinterState<'c> {
//     ctx: &'c mut GraphContext<'c>,
//     font: &'c PixelFont<'c>,
//     color: u32,
//     pixel_transformer: PixelColorTransformerFn,
//     fill_mode: ColorFillMode
// }
//
// impl<'c> PrinterState<'c> {
//     fn new(
//         ctx: &mut GraphContext,
//         font: &PixelFont,
//         color: u32,
//         pixel_transformer: PixelColorTransformerFn,
//         fill_mode: ColorFillMode
//     ) -> Self {
//         PrinterState {
//             ctx,
//             font,
//             color,
//             pixel_transformer,
//             fill_mode
//         }
//     }
//
//     fn set_color(&mut self, color:u32){
//         self.color = color;
//     }
//
//     fn set_transformer(&mut self, pixel_transformer: PixelColorTransformerFn){
//         self.pixel_transformer = pixel_transformer;
//     }
//     fn set_fill_mode(&mut self, mode:ColorFillMode){
//         self.fill_mode = mode;
//     }
//
//     fn set_font(&mut self, font:&PixelFont){
//         self.font = font;
//     }
//
// }
//
// thread_local! {
//     static STATE: RefCell<PrinterState> = RefCell::new(PrinterState::new());
// }