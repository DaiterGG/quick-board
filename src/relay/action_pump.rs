use crate::view;
use view::{action_state::Action, app::App};

pub struct ActionPump {}
impl ActionPump {
    pub fn apply(app: &mut App) {
        let actions = app.states.action.get_and_clear();
    }
}
