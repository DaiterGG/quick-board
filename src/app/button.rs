use super::{action_pump::*, input_state::*, predefined::*};

pub struct Button {}
impl Button {
    pub fn before_collision(id: Id32, input: &mut InputState) {
        if input.left() == ButtonState::Pressed {
            // NOTE: this will only register the last sub button
            // TEST: this interaciton later
            input.interacting_with = Some(id);
        }
        if input.left() == ButtonState::Released && input.interacting_with == Some(id) {
            ActionPump::add(Action::ButtonPressed(id));
        }
    }
    // pub fn after_collision(element: &UIElement, states: &mut States) {
    //     let pointer.= &mut states.pointer;
    // }
}
