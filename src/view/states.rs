use super::{
    action_state::ActionState, coords::WH, pointer_state::PointerState, ui_state::UIState,
};

pub struct States {
    pub pointer: PointerState,
    pub ui: UIState,
    pub action: ActionState,
}

impl States {
    pub fn new(window_size: WH) -> States {
        States {
            pointer: PointerState::new(),
            ui: UIState::new(window_size),
            action: ActionState::new(),
        }
    }
    pub fn reset(&mut self) {
        self.pointer.reset();
    }
}
///for testing
impl Default for States {
    fn default() -> Self {
        Self::new(WH { w: 1920, h: 1080 })
    }
}
