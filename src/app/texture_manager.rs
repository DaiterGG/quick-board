use std::{cmp::*, mem::swap, thread::sleep_ms};

use app::color_operations::ColorOperations;
use sdl2::{
    VideoSubsystem,
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::*,
    video::*,
};

use crate::*;

use super::{coords::*, texture_data::TextureData};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LockedTexId {
    IconBrush,
    IconMove,
    RangeHue,
    RangeValue,
    RangeSaturation,
    Total,
}
pub const DRAW_TEX_SIZE: u32 = 512;
pub const DRAW_TEX_SIZE_I32: i32 = DRAW_TEX_SIZE as i32;
pub struct TextureManager {
    pub t_creator: TextureCreator<WindowContext>,
    pub biggest_possible_resolution: i32,
    pub canvas: Canvas<Window>,
    // icons
    pub locked_textures: [Option<TextureData>; LockedTexId::Total as usize],
    // draw_canvas buffer, previews, displays
    pub open_textures: Vec<TextureData>,
    // DRAW_TEX_SIZExDRAW_TEX_SIZE textures, used for drawing or static for storage
    pub draw_textures: Vec<Texture>,
    // buffers, to reduce texture creations
    pub used_target_textures: Vec<usize>,
    pub unused_target_textures: Vec<usize>,
    pub unused_static_textures: Vec<usize>,
}
impl TextureManager {
    pub fn new(window: Window, bpr: i32) -> Self {
        let canvas: Canvas<Window> = CanvasBuilder::new(window)
            .build()
            .expect("Failed to create canvas");

        println!("Using SDL_Renderer \"{}\"", canvas.info().name);
        let t_creator: TextureCreator<WindowContext> = canvas.texture_creator();

        let mut p = [const { None }; Total as usize];
        use LockedTexId::*;
        use PixelFormatEnum::*;
        use TextureData as T;

        p[IconBrush as usize] = T::some(&t_creator, WH::new(32, 32));

        p[RangeHue as usize] = T::some(&t_creator, WH::new(256 * 3, 1));
        p[RangeSaturation as usize] = T::some(&t_creator, WH::new(256, 1));
        p[RangeValue as usize] = T::some(&t_creator, WH::new(256, 1));

        p[IconBrush as usize] = T::from_bytes(
            &t_creator,
            include_bytes!("../../resources/icons/brush.png"),
        );
        p[IconMove as usize] = T::from_bytes(
            &t_creator,
            include_bytes!("../../resources/icons/move_tool.png"),
        );

        Self {
            canvas,
            t_creator,
            biggest_possible_resolution: bpr,
            locked_textures: p,
            open_textures: Vec::new(),
            draw_textures: Vec::new(),
            used_target_textures: Vec::new(),
            unused_target_textures: Vec::new(),
            unused_static_textures: Vec::new(),
        }
    }
    pub fn locked_texture(&self, id: LockedTexId) -> &TextureData {
        if let Some(t) = &self.locked_textures[id as usize] {
            t
        } else {
            panic!("Texture {:?} not found", id)
        }
    }
    pub fn open_texture(&self, id: usize) -> &TextureData {
        &self.open_textures[id]
    }
    pub fn open_texture_mut(&mut self, id: usize) -> &mut TextureData {
        &mut self.open_textures[id]
    }
    /// * `size`: None - use biggest possible resolution of the pc for safety
    pub fn init_open_texture(&mut self, texture_data: TextureData) -> usize {
        self.open_textures.push(texture_data);
        self.open_textures.len() - 1
    }
    pub fn destroy_open_texture(&mut self, id: usize) {
        unsafe {
            let data = self.open_textures.remove(id);
            data.texture.destroy();
        }
    }
    pub fn init_target_texture(&mut self) -> usize {
        let new = if !self.unused_target_textures.is_empty() {
            self.unused_target_textures.pop().unwrap()
        } else {
            self.new_target_texture()
        };
        self.used_target_textures.push(new);
        new
    }
    fn init_static_texture(&mut self) -> usize {
        if !self.unused_static_textures.is_empty() {
            return self.unused_static_textures.pop().unwrap();
        }
        self.new_static_texture()
    }
    pub fn make_static(&mut self, target_id: usize) {
        let stat_id = self.init_static_texture();
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
        let mut tex = self
            .t_creator
            .create_texture_static(PixelFormatEnum::RGBA8888, DRAW_TEX_SIZE, DRAW_TEX_SIZE)
            .expect("Failed to create static texture");
        tex.set_blend_mode(BlendMode::Blend);

        self.draw_textures.push(tex);
        id
    }
    fn new_target_texture(&mut self) -> usize {
        let id = self.draw_textures.len();
        let mut tex = self
            .t_creator
            .create_texture_target(PixelFormatEnum::RGBA8888, DRAW_TEX_SIZE, DRAW_TEX_SIZE)
            .expect("Failed to create target texture");
        tex.set_blend_mode(BlendMode::Blend);

        self.draw_textures.push(tex);
        id
    }
    // static palettes
    pub fn init_palettes(&mut self) {
        let hue = self.locked_textures[LockedTexId::RangeHue as usize]
            .as_mut()
            .unwrap();
        hue.texture
            .update(None, &ColorOperations::hue_palette(), 256 * 4 * 3)
            .unwrap();
    }
    // dynamic palettes
    pub fn update_palettes(&mut self, color: Color) {
        let sat = self.locked_textures[LockedTexId::RangeSaturation as usize]
            .as_mut()
            .unwrap();
        sat.texture
            .update(None, &ColorOperations::saturation_palette(color), 256 * 4)
            .unwrap();
        let val = self.locked_textures[LockedTexId::RangeValue as usize]
            .as_mut()
            .unwrap();
        val.texture
            .update(None, &ColorOperations::value_palette(color), 256 * 4)
            .unwrap();
    }
}
