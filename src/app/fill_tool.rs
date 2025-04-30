use sdl2::{render::*, video::Window};

use super::{
    canvas_manager::CanvasData, coords::XY, history_step::HistoryStep, pointer_state::PointerState,
    texture_manager::TextureManager, tool_trait::ToolTrait,
};

pub struct Fill;
impl Fill {
    pub fn new() -> Self {
        Self {}
    }
}
impl ToolTrait for Fill {
    fn process_stroke(
        &mut self,
        data: &mut CanvasData,
        stroke_at: XY,
        pointer: &PointerState,
        canvas: &mut Canvas<Window>,
        textures: &mut TextureManager,
    ) {
        //
    }
}
