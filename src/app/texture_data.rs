use sdl2::{
    image::LoadTexture,
    pixels::{Color, PixelFormatEnum},
    render::*,
    ttf::Font,
    video::WindowContext,
};

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
}
type TC = TextureCreator<WindowContext>;
impl TextureData {
    pub fn new(
        t_creator: &TC,
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
        }
    }
    pub fn stat(t_creator: &TC, size: WH) -> TextureData {
        Self::new(t_creator, size, None, None)
    }
    pub fn some(t_creator: &TC, size: WH) -> Option<TextureData> {
        Some(Self::new(t_creator, size, None, None))
    }
    pub fn from_bytes(t_creator: &TC, bytes: &[u8]) -> Option<TextureData> {
        let texture = t_creator.load_texture_bytes(bytes).unwrap();
        Some(Self {
            texture,
            src: None,
            dst: None,
        })
    }
    pub fn with_text(font: &Font, t_creator: &TC, text: &str, color: Color) -> TextureData {
        let surface = font
            .render(text)
            .blended(color)
            .map_err(|e| e.to_string())
            .expect("Failed to render text");
        let texture = t_creator.create_texture_from_surface(&surface).unwrap();
        let dst = Some(XYWH::new(
            0,
            0,
            texture.query().width as i32,
            texture.query().height as i32,
        ));
        Self {
            texture,
            src: None,
            dst,
        }
    }
}
