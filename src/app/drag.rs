use crate::app::{action_pump::Action, action_pump::ActionPump, input_state::*, predefined::*};

pub struct Drag {}
impl Drag {
    pub fn before_collision(id: Id32, input: &mut InputState, hit: bool) {
        if hit && input.left() == ButtonState::Pressed {
            input.interacting_with = Some(id);

            input.start_holding_at = Some(input.pos);
        }
        if input.interacting_with == Some(id) {
            let delta = input.delta.to_f32().mult_one(input.mult());
            ActionPump::add(Action::Drag(id, delta));
            input.mouse_wrap_on = true;
        }
        if input.left() == ButtonState::Released && input.interacting_with == Some(id) {
            ActionPump::add(Action::DragEnd(id));
            input.mouse_wrap_on = false;
        }
    }
    // pub fn after_collision(element: &UIElement, states: &mut States) {
    // }
}
