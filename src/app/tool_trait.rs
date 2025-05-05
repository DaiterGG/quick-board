// use super::brush_tool::Brush;
// use super::canvas_manager::CanvasData;
// use super::fill_tool::Fill;
// use super::move_tool::Move;
// use super::{
//     coords::*, history_step::HistoryStep, pointer_state::PointerState,
//     texture_manager::TextureManager,
// };
// use enum_dispatch::enum_dispatch;
// use sdl2::{render::*, video::Window};

// macro_rules! tools{
//     (
//         $(
//             $tool:ident
//         ),* $(,)?
//     ) => {
//         pub enum ToolId {
//         $(
//             $tool,
//         )*
//             Total,
//         }
//         #[enum_dispatch]
//         pub enum Tool{
//         $(
//             $tool,
//         )*
//         }
//         impl Tool {
//             pub fn init_all_tools() -> Vec<Tool> {
//                 let mut vec: Vec<Tool> = Vec::new();
//                 $(
//                     vec.push($tool::new().into());
//                 )*
//                 vec
//             }
//         }
//     };
// }

// pub type ToolIdUsize = usize;
// tools! {
//     Fill,
//     Brush,
//     Move,
// }

// // #[enum_dispatch]
// // pub enum ToolsEnum {
// //     FillTool,
// //     BrushTool,
// // }
// // impl ToolsEnum {
// //     pub fn init_all_tools() -> Vec<ToolsEnum> {
// //         let mut vec: Vec<ToolsEnum> = Vec::new();
// //         vec.push(FillTool::new().into());
// //         vec.push(BrushTool::new().into());
// //         vec
// //     }
// // }
// #[enum_dispatch(Tool)]
// pub trait ToolTrait {
//     fn process_stroke(&mut self, data: ToolData);
// }
// pub struct ToolData<'a> {
//     pub c_data: &'a mut CanvasData,
//     pub pointer: &'a PointerState,
//     pub canvas: &'a mut Canvas<Window>,
//     pub t_manager: &'a mut TextureManager,
//     pub ui_transform: XYWH,
//     pub pointer_tex_space: XY,
// }
