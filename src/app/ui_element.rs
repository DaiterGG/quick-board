use crate::{
    app::{drag::Drag, slider::Slider},
    dl,
};

use super::{
    action_pump::ActionPump, button::Button, coords::XYWH, draw_window::DrawWindow, input_state::*,
    predefined::*, style_display::DisplayState, texture_manager::TextureManager, ui_map::UIMap,
};

/// UIElement can be used in a tree context,
/// but stored in a flat Vec
///
/// * `element_type`: the type of the element, can be promoted to a trait later
/// * `id`: owns a handle to itself
/// * `childrens`: list of handles to childrens, for a tree like structure
/// * `transform`: current transform, set by StyleAlign every time ui needs to be updated
pub struct UIElement {
    pub element_type: ElementType,
    pub id: Id32,
    pub childrens: Vec<Id32>,
    pub transform: XYWH,
}
#[derive(Copy, Clone)]
pub enum ElementType {
    Div,
    Button,
    DrawWindow,
    Drag,
    Slider,
    Txt,
}

impl UIElement {
    pub fn new(element: ElementType, id: Id32, childrens: Vec<Id32>) -> Self {
        Self {
            element_type: element,
            id,
            childrens,
            transform: XYWH::zero(),
        }
    }

    pub fn pointer_collision_rec(
        id: Id32,
        ui_map: &mut UIMap,
        input: &mut InputState,
        parrent_hit: bool,
    ) -> bool {
        // if parrent wasn't hit, then children are false
        let hit = parrent_hit && input.pos.is_within(ui_map.elements.get(id).transform);
        let mut was_hit_before = false;

        use ButtonState as B;
        use DisplayState as D;
        if let Some(dis) = ui_map.displays.get_mut(id) {
            was_hit_before = !hit && dis.active_states[D::Hovered as usize];
            dis.set_state(D::Hovered, hit);
            dis.set_state(D::Pressed, input.left() == B::Pressed && hit);
            dis.set_state(D::Held, input.left() == B::Held && hit);
            dis.set_state(D::Released, input.left() == B::Released && hit);
        }
        // element specific logic
        use ElementType as T;
        match ui_map.elements.get(id).element_type {
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

        for i in 0..ui_map.elements.get(id).childrens.len() {
            Self::pointer_collision_rec(ui_map.elements.get(id).childrens[i], ui_map, input, hit);
        }

        // element specific logic
        // match ui_map.elements[id as usize].element_type {
        //     T::Button if hit => {
        //         // Button::after_collision(ui.element(id), states);
        //     }
        //     _ => {} //div
        // }
        hit
    }
    pub fn draw_to(&self, styles: &UIMap, textures: &mut TextureManager) {
        let dis = &styles.displays.get(self.id);
        let color = &styles.colors;

        dis.as_ref()
            .inspect(|d| d.draw(self.transform, false, color, textures));

        for i in 0..self.childrens.len() {
            styles
                .elements
                .get(self.childrens[i])
                .draw_to(styles, textures);
        }

        dis.as_ref()
            .inspect(|d| d.draw(self.transform, true, color, textures));
    }
}
