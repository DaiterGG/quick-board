use sdl2::render::{Canvas, RenderTarget, Texture};

use super::{
    app::App, coords::XYWH, states::States, style_align::Align, style_display::DisplayState,
    style_map::StyleMap, ui_builder::Id,
};

/// element specific functions
pub trait UIElementTrait {
    fn pointer_collision(&self, states: &mut States, styles: &StyleMap);
}

pub struct UIElement {
    pub id: Id,
    pub childrens: Vec<UIElement>,
    pub transform: XYWH,
    pub element: ElementType,
}
#[derive(Copy, Clone)]
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
        let color = &styles.colors;

        dis.inspect(|d| d.draw(self.transform, false, canvas, color));

        for i in 0..self.childrens.len() {
            self.childrens[i].draw_to(canvas, styles);
        }

        dis.inspect(|d| d.draw(self.transform, true, canvas, color));
    }
    pub fn update_pos(&mut self, transform: XYWH, styles: &StyleMap, states: &mut States) {
        self.transform = transform;

        //after each child set in place, transform shrinks (for Block),
        //and next child is being applyed to a smaller window
        let mut dynamic_window = transform;
        for i in 0..self.childrens.len() {
            let new_transfrom = styles
                .get_align(self.childrens[i].id)
                .align(&mut dynamic_window, states);
            self.childrens[i].update_pos(new_transfrom, styles, states);
        }
    }
}
impl UIElementTrait for UIElement {
    fn pointer_collision(&self, states: &mut States, styles: &StyleMap) {
        if self.transform.is_within(states.pointer.x, states.pointer.y) {
            let opt = styles.get_display(self.id);
            if let Some(mut dis) = opt {
                dis.set_active(DisplayState::Hovered);
            }

            // element specific logic
            match &self.element {
                _ => {}
                ElementType::Div => {}
            }
            for i in 0..self.childrens.len() {
                self.childrens[i].pointer_collision(states, styles);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::view::{coords::XYWH, style_map::StyleMap, ui_builder::Id, ui_element::ElementType};

    use super::*;

    #[test]
    pub fn fit() {
        let win = XYWH::new(0, 0, 1000, 1000);
        let mut childs = vec![
            UIElement::new(Id::ForTest1, ElementType::Div, vec![]),
            UIElement::new(Id::ForTest2, ElementType::Div, vec![]),
        ];
        let mut div = UIElement::new(Id::ForTest1, ElementType::Div, childs);
        div.update_pos(win, &StyleMap::new(), &mut States::default());
        assert_eq!(div.childrens[0].transform.x, 0);
        assert_eq!(div.childrens[0].transform.w, 400);
        assert_eq!(div.childrens[1].transform.x, 400);
        assert_eq!(div.childrens[1].transform.w, 600);
    }
}
