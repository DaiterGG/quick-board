use sdl2::render::{Canvas, RenderTarget, Texture};

use super::{
    app::App, button::Button, coords::XYWH, pointer_state::ButtonState, states::States,
    style_align::Align, style_display::DisplayState, style_map::StyleMap, ui_builder::Id,
};

pub struct UIElement {
    pub element: ElementType,
    pub id: Id,
    pub childrens: Vec<UIElement>,
    pub transform: XYWH,
}
#[derive(Copy, Clone)]
pub enum ElementType {
    Div,
    Button,
}

impl UIElement {
    pub fn new(element: ElementType, id: Id, childrens: Vec<UIElement>) -> Self {
        Self {
            element,
            id,
            childrens,
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
    pub fn update_childrens(&mut self, styles: &StyleMap, states: &mut States) {
        //after each child set in place, transform shrinks (for Block),
        //and next child is being applied to a smaller window
        let mut dynamic_window = self.transform.clone();
        for i in 0..self.childrens.len() {
            let ch = &mut self.childrens[i];
            ch.transform = styles.get_align(ch.id).align(&mut dynamic_window, states);
            ch.update_childrens(styles, states);
        }
    }
    pub fn pointer_collision(&self, states: &mut States, styles: &mut StyleMap, parrent_hit: bool) {
        //if parrent wasn't hit, then children are not calculated
        let hit = parrent_hit && self.transform.is_within(states.pointer.x, states.pointer.y);

        let opt = styles.get_display_mut(self.id);

        if let Some(dis) = opt {
            dis.set_state(DisplayState::Hovered, hit);
            dis.set_state(
                DisplayState::Pressed,
                states.pointer.left == ButtonState::Pressed && hit,
            );
            dis.set_state(
                DisplayState::Held,
                states.pointer.left == ButtonState::Held && hit,
            );
            dis.set_state(
                DisplayState::Released,
                states.pointer.left == ButtonState::Released && hit,
            );
            println!("{:?}", dis.active_states);
            // println!("{:?}", states.pointer.left);
        }

        // element specific logic
        match &self.element {
            ElementType::Button if hit => {
                Button::before_collision(self, states);
            }
            _ => {} //div
        }

        for i in 0..self.childrens.len() {
            self.childrens[i].pointer_collision(states, styles, hit);
        }

        // element specific logic
        match &self.element {
            ElementType::Button if hit => {
                Button::after_collision(self, states);
            }
            _ => {} //div
        }
    }
}
// #[cfg(test)]
// mod tests {
//     use crate::view::{coords::XYWH, style_map::StyleMap, ui_builder::Id, ui_element::ElementType};

//     use super::*;

//     #[test]
//     pub fn fit() {
//         let win = XYWH::new(0, 0, 1000, 1000);
//         let mut childs = vec![
//             UIElement::new(Id::ForTest1, ElementType::Div, vec![]),
//             UIElement::new(Id::ForTest2, ElementType::Div, vec![]),
//         ];
//         let mut div = UIElement::new(Id::ForTest1, ElementType::Div, childs);
//         div.update_pos(win, &StyleMap::new(), &mut States::default());
//         assert_eq!(div.childrens[0].transform.x, 0);
//         assert_eq!(div.childrens[0].transform.w, 400);
//         assert_eq!(div.childrens[1].transform.x, 400);
//         assert_eq!(div.childrens[1].transform.w, 600);
//     }
// }
