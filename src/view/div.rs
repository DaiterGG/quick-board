extern crate sdl2;

use super::states::States;
use super::style_map::StyleMap;
use super::ui_builder::Id;
use super::ui_element::{UIElement, UIElementTrait};
use super::{coords::XYWH, style::Style};

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::RenderTarget;

pub struct Div {
    pub id: Id,
    pub childrens: Vec<UIElement>,
    current_transform: XYWH,
    bg_color: Option<Color>,
}

impl Div {
    pub const fn new(id: Id, bg_color: Option<Color>, childrens: Vec<UIElement>) -> Div {
        Div {
            id,
            current_transform: XYWH::new_const(0, 0, 0, 0),
            bg_color,
            childrens,
        }
    }
    // pub fn add_child(&mut self, child: Div) {
    //     self.childrens.push(child);
    // }
}
impl UIElementTrait for Div {
    fn update_pos(&mut self, transform: XYWH, styles: &StyleMap, states: &mut States) {
        self.current_transform = transform;
        Style::fit_childrens(self.current_transform, &mut self.childrens, styles, states);
    }
    fn pointer_collision(&self, states: &mut States) {
        todo!()
    }

    fn draw_to<T: RenderTarget>(&mut self, canvas: &mut Canvas<T>) {
        if let Some(background) = &self.bg_color {
            canvas.set_draw_color(*background);
            canvas
                .fill_rect(Rect::new(
                    self.current_transform.x as i32,
                    self.current_transform.y as i32,
                    self.current_transform.w as u32,
                    self.current_transform.h as u32,
                ))
                .unwrap();
        }
        for i in (0..self.childrens.len()).rev() {
            self.childrens[i].draw_to(canvas);
        }
    }
    fn get_id(&self) -> Id {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::view::{coords::WH, style::Style};

    #[test]
    pub fn fit() {
        let win = XYWH::new(0, 0, 1000, 1000);
        let mut childs = vec![
            UIElement::Div(Div::new(Id::ForTest1, None, Vec::new())),
            UIElement::Div(Div::new(Id::ForTest2, None, Vec::new())),
        ];
        Style::fit_childrens(
            win,
            &mut childs,
            &StyleMap::new(vec![]),
            &mut States::new(WH { w: 1000, h: 1000 }),
        );
        let div1 = childs[0].unwrap_div();
        let div2 = childs[1].unwrap_div();
        assert_eq!(div1.current_transform.x, 0);
        assert_eq!(div1.current_transform.w, 400);
        assert_eq!(div2.current_transform.x, 400);
        assert_eq!(div2.current_transform.w, 600);
    }
}
