use sdl2::{render::*, video::Window};

use super::{
    canvas_manager::CanvasData, coords::XY, history_step::HistoryStep, pointer_state::PointerState,
    texture_manager::TextureManager,
};

pub struct Fill;
impl Fill {
    pub fn new() -> Self {
        Self {}
    }
    pub fn process_stroke(
        &mut self,
        data: &mut CanvasData,
        pointer: &PointerState,
        canvas: &mut Canvas<Window>,
        textures: &mut TextureManager,
    ) {
        //
    }
}
