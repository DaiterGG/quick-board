use crate::app::{drag::Drag, slider::Slider};

use super::{
    action_pump::ActionPump, button::Button, coords::XYWH, draw_window::DrawWindow, input_state::*,
    predefined::*, style_display::DisplayState, texture_manager::TextureManager, ui_map::UIMap,
};

pub struct UIElement {
    pub element_type: ElementType,
    pub id: IdI32,
    pub childrens: Vec<IdI32>,
    pub transform: XYWH,
}
#[derive(Copy, Clone)]
pub enum ElementType {
    Div,
    Button,
    DrawWindow,
    Drag,
    Slider,
}

impl UIElement {
    pub fn new(element: ElementType, id: IdI32, childrens: Vec<IdI32>) -> Self {
        Self {
            element_type: element,
            id,
            childrens,
            transform: XYWH::zero(),
        }
    }

    pub fn pointer_collision_rec(
        id: IdI32,
        ui_map: &mut UIMap,
        input: &mut InputState,
        parrent_hit: bool,
    ) -> bool {
        // if parrent wasn't hit, then children are false
        let hit = parrent_hit && input.pos.is_within(ui_map.elements[id as usize].transform);
        let mut was_hit_before = false;

        use ButtonState as B;
        use DisplayState as D;
        if let Some(dis) = &mut ui_map.displays[id as usize] {
            was_hit_before = !hit && dis.active_states[D::Hovered as usize];
            dis.set_state(D::Hovered, hit);
            dis.set_state(D::Pressed, input.left() == B::Pressed && hit);
            dis.set_state(D::Held, input.left() == B::Held && hit);
            dis.set_state(D::Released, input.left() == B::Released && hit);
            // println!("{:?}", dis.active_states);
            // println!("{:?}", states.pointer.left);
        }
        // element specific logic
        use ElementType as T;
        match ui_map.elements[id as usize].element_type {
            T::Button if hit => {
                Button::before_collision(id, input);
            }
            T::DrawWindow => {
                DrawWindow::before_collision(id, input, ui_map, hit, was_hit_before);
            }
            T::Drag => {
                Drag::before_collision(id, input, hit);
            }
            T::Slider => {
                Slider::before_collision(id, input, ui_map, hit);
            }
            _ => {} //div
        }

        for i in 0..ui_map.elements[id as usize].childrens.len() {
            Self::pointer_collision_rec(
                ui_map.elements[id as usize].childrens[i],
                ui_map,
                input,
                hit,
            );
        }

        // element specific logic
        // match ui_map.elements[id as usize].element_type {
        //     T::Button if hit => {
        //         // Button::after_collision(ui.element(id), states);
        //     }
        //     _ => {} //div
        // }
        // for ui_manager check
        hit
    }
    pub fn draw_to(&self, styles: &UIMap, textures: &mut TextureManager) {
        let dis = &styles.displays[self.id as usize];
        let color = &styles.colors;

        dis.as_ref()
            .inspect(|d| d.draw(self.transform, false, color, textures));

        for i in 0..self.childrens.len() {
            styles.elements[self.childrens[i] as usize].draw_to(styles, textures);
        }

        dis.as_ref()
            .inspect(|d| d.draw(self.transform, true, color, textures));
    }
}
// #[cfg(test)]
// impl Default for UIElement {
//     fn default() -> Self {
//         Self {
//             element_type: ElementType::Div,
//             id: Id::ForTest1 as usize,
//             childrens: Vec::new(),
//             transform: XYWH::default(),
//         }
//     }
// }
// #[cfg(test)]
// mod tests {
//     use crate::view::{coords::XYWH, ui_element::ElementType};

//     use super::*;

//     #[test]
//     pub fn fit() {
//         let win = XYWH::new(0, 0, 1000, 1000);
//         let mut childs = vec![
//             UIElement::new(Id::ForTest1 as usize, ElementType::Div, vec![]),
//             UIElement::new(Id::ForTest2 as usize, ElementType::Div, vec![]),
//         ];
//         let mut div = UIElement::new(Id::ForTest1, ElementType::Div, childs);
//         UIElement::update(
//             Id::ForTest1 as usize,
//             win,
//             &mut UIMap::new(),
//             &mut States::default(),
//         );
//         assert_eq!(div.childrens[0].transform.x, 0);
//         assert_eq!(div.childrens[0].transform.w, 400);
//         assert_eq!(div.childrens[1].transform.x, 400);
//         assert_eq!(div.childrens[1].transform.w, 600);
//     }
// }
