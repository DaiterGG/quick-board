use std::mem::replace;

use super::ui_builder::Id;

pub struct ActionState {
    pub actions: Vec<Action>,
}

pub enum Action {
    ButtonPressed(Id),
}

impl ActionState {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn add(&mut self, action: Action) {
        self.actions.push(action);
    }
    pub fn get_and_clear(&mut self) -> Vec<Action> {
        replace(&mut self.actions, Vec::new())
    }

    pub fn reset(&mut self) {}
}
