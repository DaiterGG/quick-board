use super::{action_pump::*, input_state::*, predefined::*, ui_element::UIElement};

pub struct DrawWindow;
impl DrawWindow {
    pub fn before_collision(
        id: IdI32,
        element: &UIElement,
        actions: &mut ActionPump,
        input: &mut InputState,
        was_hit: bool,
    ) {
        if was_hit && input.left() == ButtonState::Pressed {
            input.interacting_with = Some(id);
        }
        // if !was_hit && pointer.interacting_with == Some(id) {
        //     pointer.interacting_with = None;
        // }
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
