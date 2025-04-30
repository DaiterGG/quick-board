use super::{
    action_pump::{Action, ActionPump},
    coords::XY,
    pointer_state::{self, ButtonState, PointerState},
    predefined::{Id, IdUsize},
    ui_element::UIElement,
};

pub struct DrawWindow;
impl DrawWindow {
    pub fn before_collision(
        id: IdUsize,
        element: &UIElement,
        actions: &mut ActionPump,
        pointer: &mut PointerState,
        was_hit: bool,
    ) {
        if was_hit && pointer.left == ButtonState::Pressed {
            pointer.interacting_with = Some(id);
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
