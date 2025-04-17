use std::i32;

use sdl2::render::{Canvas, RenderTarget};

use super::coords::{WH, XY, XYWH};
use super::states::States;
use super::style_map::StyleMap;
use super::ui_element::{UIElement, UIElementTrait};

// potential alternatives
// display_data: Box<dyn Any>, code overhead for casting types
// Style<T: DisplayType>, div need to know store type specific style
// or with duplicate Style {display_absolute, display_block} 👌
// pub struct Style(Type);

#[derive(Copy, Clone)]
pub enum Display {
    None,
}

impl Display {
    pub fn draw_back<T: RenderTarget>(&self, pos: XYWH, canvas: &mut Canvas<T>) {
        //
        // if let Some(background) = &self.bg_color {
        //     canvas.set_draw_color(*background);
        //     canvas.fill_rect(Rect::new()).unwrap();
        // }
    }
    pub fn draw_front<T: RenderTarget>(&self, pos: XYWH, canvas: &mut Canvas<T>) {
        //
    }
}
