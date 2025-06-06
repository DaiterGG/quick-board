use std::sync::OnceLock;

use crate::app::{
    texture_data::TextureData,
    texture_vec::{TexId16, TextureVec},
};
use sdl2::{
    pixels::{Color, PixelFormatEnum},
    render::*,
    rwops::RWops,
    sys::{SDL_ComposeCustomBlendMode, SDL_SetTextureBlendMode},
    ttf::{Font, Sdl2TtfContext},
    video::*,
};

pub static TTF_CONTEXT: OnceLock<Sdl2TtfContext> = OnceLock::new();

pub const DRAW_TEX_SIZE: u32 = 512;
pub const DRAW_TEX_SIZE_I32: i32 = DRAW_TEX_SIZE as i32;

pub struct TextureManager<'a> {
    pub t_creator: TextureCreator<WindowContext>,
    pub biggest_possible_resolution: i32,
    pub canvas: Canvas<Window>,

    // pub ttf_context: Sdl2TtfContext,
    // pub main_font_buffer: Fonts<'a>,
    pub main_font_raw: &'a [u8],
    pub main_font_buffer: Font<'a, 'a>,
    pub main_font_buffer_size: u16,

    pub textures: TextureVec,

    // DRAW_TEX_SIZExDRAW_TEX_SIZE textures, used for drawing or static for storage
    // TODO: Move is separate struct
    pub draw_textures: Vec<Texture>,
    // buffers, to reduce texture creations
    pub used_target_textures: Vec<usize>,
    pub unused_target_textures: Vec<usize>,
    pub unused_static_textures: Vec<usize>,
}
impl TextureManager<'_> {
    pub fn new(window: Window, bpr: i32) -> Self {
        TTF_CONTEXT
            .set(sdl2::ttf::init().expect("Failed to init ttf"))
            .unwrap_or_else(|_| panic!("Failed to set static ttf"));

        let canvas: Canvas<Window> = CanvasBuilder::new(window)
            .build()
            .expect("Failed to create canvas");

        println!("Using SDL_Renderer \"{}\"", canvas.info().name);
        let t_creator: TextureCreator<WindowContext> = canvas.texture_creator();

        let raw = include_bytes!("../../resources/fonts/Inter/Inter-VariableFont_opsz,wght.ttf");
        let rwops = RWops::from_bytes(raw).expect("Failed to load font");

        let main_font_buffer_size = 12;
        let main_font_buffer = TTF_CONTEXT
            .get()
            .unwrap()
            .load_font_from_rwops(rwops, main_font_buffer_size)
            .expect("Failed to load font");

        let textures = TextureVec::new(&t_creator);
        Self {
            canvas,
            t_creator,
            biggest_possible_resolution: bpr,
            main_font_raw: raw,
            main_font_buffer,
            main_font_buffer_size,
            textures,
            draw_textures: Vec::new(),
            used_target_textures: Vec::new(),
            unused_target_textures: Vec::new(),
            unused_static_textures: Vec::new(),
        }
    }

    pub fn init_target_texture(&mut self, is_eraser: bool) -> usize {
        let new = if !self.unused_target_textures.is_empty() {
            self.unused_target_textures.pop().unwrap()
        } else {
            self.new_target_texture()
        };
        self.set_draw_blend_mode(new, is_eraser);
        self.used_target_textures.push(new);
        new
    }
    fn init_static_texture(&mut self, is_eraser: bool) -> usize {
        if !self.unused_static_textures.is_empty() {
            let to_use = self.unused_static_textures.pop().unwrap();
            self.set_draw_blend_mode(to_use, is_eraser);
            return to_use;
        }
        let new = self.new_static_texture();
        self.set_draw_blend_mode(new, is_eraser);
        new
    }
    pub fn make_static(&mut self, target_id: usize, is_eraser: bool) {
        let stat_id = self.init_static_texture(is_eraser);
        let target = &mut self.draw_textures[target_id];
        let mut pixel_data = Vec::new();
        self.canvas
            .with_texture_canvas(target, |c| {
                pixel_data = c.read_pixels(None, PixelFormatEnum::RGBA8888).unwrap();
                c.set_draw_color(Color::RGBA(0, 0, 0, 0));
                c.clear();
            })
            .expect("Failed to read pixels");

        let stat = &mut self.draw_textures[stat_id];
        stat.update(None, pixel_data.as_slice(), DRAW_TEX_SIZE as usize * 4)
            .expect("Failed to update static texture");

        self.draw_textures.swap(target_id, stat_id);

        // stat_id is now target_id, target is not used
        self.unused_target_textures.push(stat_id);
        self.used_target_textures.retain(|x| *x != target_id);
    }
    pub fn buffer_draw_texture(&mut self) {
        if self.unused_target_textures.len() < 10 {
            //bruh
            let id = self.new_target_texture();
            self.unused_target_textures.push(id);
        }
        if self.used_target_textures.len() > self.unused_static_textures.len() {
            let id = self.new_static_texture();
            self.unused_static_textures.push(id);
        }
    }
    fn new_static_texture(&mut self) -> usize {
        let id = self.draw_textures.len();
        let tex = self
            .t_creator
            .create_texture_static(PixelFormatEnum::RGBA8888, DRAW_TEX_SIZE, DRAW_TEX_SIZE)
            .expect("Failed to create static texture");

        self.draw_textures.push(tex);
        id
    }
    fn new_target_texture(&mut self) -> usize {
        let id = self.draw_textures.len();
        let tex = self
            .t_creator
            .create_texture_target(PixelFormatEnum::RGBA8888, DRAW_TEX_SIZE, DRAW_TEX_SIZE)
            .expect("Failed to create target texture");

        self.draw_textures.push(tex);
        id
    }
    fn set_draw_blend_mode(&mut self, tex_id: usize, is_eraser: bool) {
        let tex = &self.draw_textures[tex_id];
        if is_eraser {
            unsafe {
                let custom = SDL_ComposeCustomBlendMode(
                    sdl2::sys::SDL_BlendFactor::SDL_BLENDFACTOR_ZERO,
                    sdl2::sys::SDL_BlendFactor::SDL_BLENDFACTOR_ONE,
                    sdl2::sys::SDL_BlendOperation::SDL_BLENDOPERATION_ADD,
                    sdl2::sys::SDL_BlendFactor::SDL_BLENDFACTOR_ONE,
                    sdl2::sys::SDL_BlendFactor::SDL_BLENDFACTOR_ONE,
                    sdl2::sys::SDL_BlendOperation::SDL_BLENDOPERATION_REV_SUBTRACT,
                );
                SDL_SetTextureBlendMode(tex.raw(), custom);
            }
            return;
        }
        unsafe {
            let custom = SDL_ComposeCustomBlendMode(
                //color similar to BlendMode::Blend, but do not change src color
                sdl2::sys::SDL_BlendFactor::SDL_BLENDFACTOR_ONE,
                sdl2::sys::SDL_BlendFactor::SDL_BLENDFACTOR_ONE_MINUS_SRC_ALPHA,
                sdl2::sys::SDL_BlendOperation::SDL_BLENDOPERATION_ADD,
                sdl2::sys::SDL_BlendFactor::SDL_BLENDFACTOR_ONE,
                sdl2::sys::SDL_BlendFactor::SDL_BLENDFACTOR_ONE,
                sdl2::sys::SDL_BlendOperation::SDL_BLENDOPERATION_MAXIMUM,
            );
            SDL_SetTextureBlendMode(tex.raw(), custom);
        }
    }
    fn try_update_font_size(&mut self, size: u16) {
        if self.main_font_buffer_size != size {
            self.main_font_buffer = TTF_CONTEXT
                .get()
                .unwrap()
                .load_font_from_rwops(
                    RWops::from_bytes(self.main_font_raw).expect("Failed to load font"),
                    size,
                )
                .unwrap();
            self.main_font_buffer_size = size;
        }
    }
    pub fn new_text_texture(
        &mut self,
        text: &str,
        size: u16,
        color: Color,
    ) -> (TexId16, (u32, u32)) {
        self.try_update_font_size(size);
        let text_size = self.main_font_buffer.size_of(text).unwrap();
        let tex_d = if text_size.0 == 0 || text_size.1 == 0 {
            TextureData::with_text(&self.main_font_buffer, &self.t_creator, " ", color)
        } else {
            TextureData::with_text(&self.main_font_buffer, &self.t_creator, text, color)
        };
        (self.textures.init_texture(tex_d), text_size)
    }
}
