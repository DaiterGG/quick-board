use super::ui_builder::Id;

pub struct ActionState {
    pub actions: Vec<String>,
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

    pub fn reset(&mut self) {}
}
