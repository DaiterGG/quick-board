use crate::{TextureManager, app::action_pump::Action, dl};

use super::{action_pump::ActionPump, input_state::*, predefined::*, ui_map::UIMap};

pub struct Drag {}
// impl Button {
//     pub const fn new(id: Id, bg_color: Option<Color>) -> Div {
//     }
// }
impl Drag {
    pub fn before_collision(id: IdI32, input: &mut InputState, hit: bool) {
        if hit && input.left() == ButtonState::Pressed {
            input.interacting_with = Some(id);

            input.start_holding_at = Some(input.pos);
        }
        if input.interacting_with == Some(id) {
            let delta = input.delta.to_f32().mult_one(input.mult());
            ActionPump::add(Action::Drag(id, delta));
            input.mouse_wrap_on = true;
        } else {
            input.mouse_wrap_on = false;
        }
    }
    // pub fn after_collision(element: &UIElement, states: &mut States) {
    // }
}
