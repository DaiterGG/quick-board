use sdl2::{render::*, video::WindowContext};

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
    pub size: WH,
}
impl TextureData {
    pub fn new(t_creator: &TextureCreator<WindowContext>, size: WH) -> TextureData {
        let mut texture = t_creator
            .create_texture_target(
                t_creator.default_pixel_format(),
                size.w as u32,
                size.h as u32,
            )
            .unwrap();
        texture.set_blend_mode(BlendMode::Blend);

        Self {
            texture,
            src: None,
            dst: None,
            size,
        }
    }
    pub fn some(t_creator: &TextureCreator<WindowContext>, size: WH) -> Option<TextureData> {
        Some(Self::new(t_creator, size))
    }
}
