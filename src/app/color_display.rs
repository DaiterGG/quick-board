use sdl2::render::BlendMode::Blend;
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use super::{
    color_map::{ColorMap, ColorTag},
    coords::XYWH,
};

#[derive(Copy, Clone, Debug)]
pub struct ColorDisplay {
    color: ColorTag,
    alfa: u8,
}
impl ColorDisplay {
    pub fn full(color: ColorTag) -> Self {
        Self { color, alfa: 255 }
    }
    pub fn with_alfa(color: ColorTag, alfa: u8) -> Self {
        Self { color, alfa }
    }
    pub fn apply(&self, canvas: &mut Canvas<Window>, colors: &ColorMap, pos: XYWH) {
        let mut rgba: Color = colors.get(self.color);
        rgba.a = self.alfa as u8;
        canvas.set_draw_color(rgba);
        canvas.set_blend_mode(Blend);
        let _ = canvas.fill_rect(Rect::new(pos.x, pos.y, pos.w as u32, pos.h as u32));
    }
}
