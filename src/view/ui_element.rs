use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::Color,
    render::{Canvas, RenderTarget, Texture},
};

use super::{
    app::App, coords::XYWH, div::Div, states::States, style_align::Align, style_map::StyleMap,
    ui_builder::Id,
};

/// element specific functions
pub trait UIElementTrait {
    fn pointer_collision(&self, states: &mut States);
}

pub struct UIElement {
    pub id: Id,
    pub childrens: Vec<UIElement>,
    pub transform: XYWH,
    pub element: ElementType,
}
pub enum ElementType {
    Div,
}
impl UIElement {
    pub fn new(id: Id, element: ElementType, childrens: Vec<UIElement>) -> Self {
        Self {
            id,
            childrens,
            element,
            transform: XYWH::default(),
        }
    }
    pub fn draw_to<T: RenderTarget>(&self, canvas: &mut Canvas<T>, styles: &StyleMap) {
        let dis = styles.get_display(self.id);
        dis.draw_back(self.transform, canvas);
        for i in 0..self.childrens.len() {
            self.childrens[i].draw_to(canvas, styles);
        }
        dis.draw_front(self.transform, canvas);
    }
    pub fn update_pos(&mut self, transform: XYWH, styles: &StyleMap, states: &mut States) {
        self.transform = transform;
        Align::fit_childrens(self.transform, &mut self.childrens, styles, states);
    }
}
impl UIElementTrait for UIElement {
    fn pointer_collision(&self, states: &mut States) {
        match &self.element {
            _ => {}
            ElementType::Div => {
                for i in 0..self.childrens.len() {
                    self.childrens[i].pointer_collision(states);
                }
            }
        }
    }
}
// #[cfg(test)]
// impl UIElement {
//     pub fn unwrap_div(&self) -> &Div {
//         match &self.element {
//             ElementType::Div(div) => div,
//         }
//     }
// }
#[cfg(test)]
mod tests {
    use crate::view::{
        coords::{WH, XYWH},
        style_align::Align,
        style_map::StyleMap,
        ui_builder::Id,
        ui_element::ElementType,
    };

    use super::*;

    #[test]
    pub fn fit() {
        let win = XYWH::new(0, 0, 1000, 1000);
        let mut childs = vec![UIElement::new(Id::ForTest1, ElementType::Div, vec![])];
        Align::fit_childrens(
            win,
            &mut childs,
            &StyleMap::new(),
            &mut States::new(WH { w: 1000, h: 1000 }),
        );
        assert_eq!(childs[0].transform.x, 0);
        assert_eq!(childs[0].transform.w, 400);
        assert_eq!(childs[1].transform.x, 400);
        assert_eq!(childs[1].transform.w, 600);
    }
}
