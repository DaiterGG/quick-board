use crate::{app::action_pump::Action, dl};

use super::{action_pump::ActionPump, input_state::*, predefined::*, ui_map::UIMap};

pub struct Slider {
    pub handle_id: IdI32,
}
impl Slider {
    pub fn new(handle_id: IdI32) -> Self {
        Self { handle_id }
    }
    pub fn before_collision(
        id: IdI32,
        actions: &mut ActionPump,
        input: &mut InputState,
        ui: &UIMap,
        hit: bool,
    ) {
        let this = &ui.sliders_data[&id];
        let handle_a = &ui.aligns[this.handle_id as usize];
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
