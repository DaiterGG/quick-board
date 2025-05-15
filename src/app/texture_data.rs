use sdl2::{pixels::PixelFormatEnum, render::*, video::WindowContext};

use super::coords::*;

/// mainly to use in the ui
///
/// * `texture`: texture to .copy()
/// * `src`: None - stretch texture
/// * `dst`: None - full texture
/// * `size`: texture original size
pub struct TextureData {
    pub texture: Texture,
    pub src: Option<XYWH>,
    pub dst: Option<XYWH>,
    // pub size: WH,
}
impl TextureData {
    pub fn new(
        t_creator: &TextureCreator<WindowContext>,
        size: WH,
        format: Option<PixelFormatEnum>,
    ) -> TextureData {
        let format = format.unwrap_or(t_creator.default_pixel_format());
        let texture = t_creator
            .create_texture_target(format, size.w as u32, size.h as u32)
            .unwrap();

        Self {
            texture,
            src: None,
            dst: None,
            // size,
        }
    }
    pub fn some(t_creator: &TextureCreator<WindowContext>, size: WH) -> Option<TextureData> {
        Some(Self::new(t_creator, size, None))
    }
}
