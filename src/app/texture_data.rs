use sdl2::{image::LoadTexture, pixels::PixelFormatEnum, render::*, video::WindowContext};

use crate::dl;

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
        access: Option<TextureAccess>,
    ) -> TextureData {
        let format = format.unwrap_or(t_creator.default_pixel_format());
        let access = access.unwrap_or(TextureAccess::Static);
        let texture = t_creator
            .create_texture(format, access, size.w as u32, size.h as u32)
            .unwrap();

        Self {
            texture,
            src: None,
            dst: None,
            // size,
        }
    }
    pub fn some(t_creator: &TextureCreator<WindowContext>, size: WH) -> Option<TextureData> {
        Some(Self::new(t_creator, size, None, None))
    }
    pub fn from_bytes(
        t_creator: &TextureCreator<WindowContext>,
        bytes: &[u8],
        // size: WH,
    ) -> Option<TextureData> {
        let texture = t_creator.load_texture_bytes(bytes).unwrap();
        Some(Self {
            texture,
            src: None,
            dst: None,
            // size,
        })
    }
}
