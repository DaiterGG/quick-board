use super::{action_pump::*, pointer_state::*, predefined::*, ui_element::UIElement};

pub struct DrawWindow;
impl DrawWindow {
    pub fn before_collision(
        id: IdI32,
        element: &UIElement,
        actions: &mut ActionPump,
        pointer: &mut PointerState,
        was_hit: bool,
    ) {
        if was_hit && pointer.left == ButtonState::Pressed {
            pointer.interacting_with = Some(id);
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
