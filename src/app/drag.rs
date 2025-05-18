use crate::{app::action_pump::Action, dl};

use super::{action_pump::ActionPump, input_state::*, predefined::*, ui_map::UIMap};

pub struct Drag {}
// impl Button {
//     pub const fn new(id: Id, bg_color: Option<Color>) -> Div {
//     }
// }
impl Drag {
    pub fn before_collision(
        id: IdI32,
        actions: &mut ActionPump,
        input: &mut InputState,
        hit: bool,
    ) {
        if hit && input.left() == ButtonState::Pressed {
            input.interacting_with = Some(id);

            input.start_holding_at = Some(input.pos);
        }
        if input.interacting_with == Some(id) {
            let delta = input.delta.to_f32().mult_one(input.mult());
            actions.add(Action::Drag(id, delta));
        }
    }
    // pub fn after_collision(element: &UIElement, states: &mut States) {
    // }
}
