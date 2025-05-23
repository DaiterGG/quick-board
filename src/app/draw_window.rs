use super::{action_pump::*, input_state::*, predefined::*, ui_element::UIElement, ui_map::UIMap};

pub struct DrawWindow;
impl DrawWindow {
    pub fn before_collision(
        id: IdI32,
        input: &mut InputState,
        ui: &UIMap,
        was_hit: bool,
        was_hit_before: bool,
    ) {
        if was_hit {
            ActionPump::add(Action::CursorInCanvas(true));

            if input.left() == ButtonState::Pressed {
                input.interacting_with = Some(id);
            }
        }
        if was_hit_before && input.left() == ButtonState::Idle {
            ActionPump::add(Action::CursorInCanvas(false));
        }
        if !was_hit && input.left() == ButtonState::Released {
            ActionPump::add(Action::CursorInCanvas(false));
        }
        // if pointer.interacting_with == Some(id) {
        //     let canvas_hit = XY {
        //         x: pointer.pos.x - element.transform.x,
        //         y: pointer.pos.y - element.transform.y,
        //     };
        //     actions.add(Action::CanvasPressed(canvas_hit));
        // }
    }
    // pub fn after_collision(element: &UIElement, states: &mut States) {
    //     let pntr = &mut states.pointer;
    // }
}
