use sdl2::{rect::Rect, render::Canvas, video::Window};

use super::{
    color_map::{ColorMap, ColorTag},
    coords::XYWH,
};

#[derive(Copy, Clone, Debug)]
/// * `width`: (top, right, bottom, left)
pub struct Border {
    color: ColorTag,
    width: (u8, u8, u8, u8),
}
impl Border {
    pub fn single_w(color: ColorTag, width: u8) -> Self {
        Self {
            color,
            width: (width, width, width, width),
        }
    }
    /// * `width`: (top, right, bottom, left)
    pub fn all_w(color: ColorTag, width: (u8, u8, u8, u8)) -> Self {
        Self { color, width }
    }
    pub fn apply(&self, canvas: &mut Canvas<Window>, colors: &ColorMap, pos: XYWH) {
        canvas.set_draw_color(colors.get(self.color));
        canvas
            .fill_rects(&[
                Rect::new(pos.x, pos.y, pos.w as u32, self.width.0 as u32),
                Rect::new(
                    pos.x + pos.w - self.width.1 as i32,
                    pos.y,
                    self.width.1 as u32,
                    pos.h as u32,
                ),
                Rect::new(
                    pos.x,
                    pos.y + pos.h - self.width.2 as i32,
                    pos.w as u32,
                    self.width.2 as u32,
                ),
                Rect::new(pos.x, pos.y, self.width.3 as u32, pos.h as u32),
            ])
            .unwrap();
    }
}
