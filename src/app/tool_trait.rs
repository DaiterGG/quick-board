use super::brush_tool::Brush;
use super::canvas_manager::CanvasData;
use super::fill_tool::Fill;
use super::{
    coords::*, history_step::HistoryStep, pointer_state::PointerState,
    texture_manager::TextureManager,
};
use enum_dispatch::enum_dispatch;
use sdl2::{render::*, video::Window};

macro_rules! tools{
    (
        $(
            $tool:ident
        ),* $(,)?
    ) => {
        pub enum ToolId {
        $(
            $tool,
        )*
            Total,
        }
        #[enum_dispatch]
        pub enum Tool{
        $(
            $tool,
        )*
        }
        impl Tool {
            pub fn init_all_tools() -> Vec<Tool> {
                let mut vec: Vec<Tool> = Vec::new();
                $(
                    vec.push($tool::new().into());
                )*
                vec
            }
        }
    };
}

pub type ToolIdUsize = usize;
tools! {
    Fill,
    Brush,
}

// #[enum_dispatch]
// pub enum ToolsEnum {
//     FillTool,
//     BrushTool,
// }
// impl ToolsEnum {
//     pub fn init_all_tools() -> Vec<ToolsEnum> {
//         let mut vec: Vec<ToolsEnum> = Vec::new();
//         vec.push(FillTool::new().into());
//         vec.push(BrushTool::new().into());
//         vec
//     }
// }
#[enum_dispatch(Tool)]
pub trait ToolTrait {
    fn process_stroke(
        &mut self,
        canvas_data: &mut CanvasData,
        stroke_at: XY,
        pointer: &PointerState,
        canvas: &mut Canvas<Window>,
        t_manager: &mut TextureManager,
    );
}
