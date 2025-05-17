use crate::app::action_pump::Action;

use super::{action_pump::ActionPump, input_state::*, predefined::*};

pub struct Button {}
// impl Button {
//     pub const fn new(id: Id, bg_color: Option<Color>) -> Div {
//     }
// }
impl Button {
    pub fn before_collision(id: IdI32, actions: &mut ActionPump, input: &mut InputState) {
        if input.left() == ButtonState::Pressed {
            // NOTE: this will only register the last sub button
            // TEST: this interaciton later
            input.interacting_with = Some(id);

            // println!("interacting ");
        }
        if input.left() == ButtonState::Released && input.interacting_with == Some(id) {
            actions.add(Action::ButtonPressed(id));
            // println!("action registered")
        }
    }
    // pub fn after_collision(element: &UIElement, states: &mut States) {
    //     let pointer.= &mut states.pointer;
    // }
}
