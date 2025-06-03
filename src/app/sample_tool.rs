use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
};

use crate::{TextureManager, dl};

use super::{canvas_manager::CanvasData, coords::*, input_state::InputState};

pub struct Sample {}
impl Sample {
    pub fn new() -> Self {
        Self {}
    }
    pub fn process_stroke(
        &mut self,
        data: &mut CanvasData,
        input: &InputState,
        t_manager: &mut TextureManager,
    ) {
        if input.interacting_with != Some(data.targeted_ui_element) {
            return;
        }
        let tex = &mut t_manager.textures.get_mut(data.targeted_ui_texture).texture;
        let mut pixel = Vec::new();
        t_manager
            .canvas
            .with_texture_canvas(tex, |c| {
                pixel = c
                    .read_pixels(
                        Rect::new(input.pos.x, input.pos.y, 1, 1),
                        PixelFormatEnum::RGB888,
                    )
                    .unwrap();
            })
            .unwrap();
        data.color.set(Color::RGB(pixel[2], pixel[1], pixel[0]));
    }
}
