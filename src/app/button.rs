use crate::app::action_pump::Action;

use super::{action_pump::ActionPump, pointer_state::*, predefined::*};

pub struct Button {}
// impl Button {
//     pub const fn new(id: Id, bg_color: Option<Color>) -> Div {
//     }
// }
impl Button {
    pub fn before_collision(id: IdI32, actions: &mut ActionPump, pointer: &mut PointerState) {
        if pointer.left == ButtonState::Pressed {
            // NOTE: this will only register the last sub button
            // TEST: this interaciton later
            pointer.interacting_with = Some(id);

            // println!("interacting ");
        }
        if pointer.left == ButtonState::Released && pointer.interacting_with == Some(id) {
            actions.add(Action::ButtonPressed(id));
            // println!("action registered")
        }
    }
    // pub fn after_collision(element: &UIElement, states: &mut States) {
    //     let pointer.= &mut states.pointer;
    // }
}
