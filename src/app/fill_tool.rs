use sdl2::{render::*, video::Window};

use super::{canvas_manager::CanvasData, input_state::InputState, texture_manager::TextureManager};

pub struct Fill;
impl Fill {
    pub fn new() -> Self {
        Self {}
    }
    pub fn process_stroke(
        &mut self,
        data: &mut CanvasData,
        input: &InputState,
        canvas: &mut Canvas<Window>,
        textures: &mut TextureManager,
    ) {
        //
    }
}
