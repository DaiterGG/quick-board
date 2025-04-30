use std::cmp::*;

use sdl2::{VideoSubsystem, render::*, video::WindowContext};

use super::{coords::*, texture_data::TextureData};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LockedTexId {
    IconBrush,
    Total,
}
pub const DRAW_TEX_SIZE: u32 = 256;
pub struct TextureManager {
    t_creator: TextureCreator<WindowContext>,
    biggest_possible_resolution: WH,
    // icons
    pub locked_textures: [Option<TextureData>; LockedTexId::Total as usize],
    // draw_canvas buffer, previews, displays
    pub open_textures: Vec<TextureData>,
    // DRAW_TEX_SIZExDRAW_TEX_SIZE textures, used for drawing
    pub draw_textures: Vec<Texture>,
    // buffer, to reduce texture creations
    pub unused_draw_textures: Vec<usize>,
}
impl TextureManager {
    pub fn new(t_creator: TextureCreator<WindowContext>, video_subsystem: &VideoSubsystem) -> Self {
        let big = Self::init_biggest_possible_display_res(video_subsystem);
        let mut p = [const { None }; LockedTexId::Total as usize];
        p[LockedTexId::IconBrush as usize] = TextureData::some(&t_creator, WH::new(32, 32));

        Self {
            t_creator,
            biggest_possible_resolution: big,
            locked_textures: p,
            open_textures: Vec::new(),
            draw_textures: Vec::new(),
            unused_draw_textures: Vec::new(),
        }
    }
    fn init_biggest_possible_display_res(video_subsystem: &VideoSubsystem) -> WH {
        // reasonable minimum values
        let mut max_wh = WH { w: 1000, h: 500 };
        let count = video_subsystem.num_video_displays().unwrap_or(1);
        for i in 0..count {
            // TODO: find what mode_index is
            if let Ok(res) = video_subsystem.display_mode(i, 0) {
                if res.w > max_wh.w {
                    max_wh.w = res.w;
                }
                if res.h > max_wh.h {
                    max_wh.h = res.h;
                }
            }
        }
        max_wh
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
    pub fn init_mut_texture(&mut self, size: Option<WH>) -> usize {
        if let Some(s) = size {
            self.open_textures
                .push(TextureData::new(&self.t_creator, s));
        } else {
            self.open_textures.push(TextureData::new(
                &self.t_creator,
                self.biggest_possible_resolution,
            ));
        }
        self.open_textures.len() - 1
    }
    pub fn draw_texture(&mut self, id: usize) -> &mut Texture {
        &mut self.draw_textures[id]
    }
    pub fn init_draw_texture(&mut self) -> usize {
        if self.unused_draw_textures.len() > 0 {
            return self.unused_draw_textures.pop().unwrap();
        }
        let id = self.draw_textures.len();
        let res = self
            .t_creator
            .create_texture_target(None, DRAW_TEX_SIZE, DRAW_TEX_SIZE);

        self.draw_textures.push(res.unwrap_or_else(|e| {
            panic!("Failed to create draw texture: {}", e);
        }));
        id
    }
}
